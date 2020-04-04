use crate::modules::live_data_processor::dto::{Message, LiveDataProcessorFailure, MessageType};
use crate::modules::live_data_processor::material::Server;
use crate::modules::live_data_processor::domain_value::{SpellCast, Event, EventType, HitType, Damage, School, Mitigation, Heal};
use crate::modules::live_data_processor::tools::MapUnit;

pub trait ParseEvents {
  fn parse_events(&mut self, messages: Vec<Message>) -> Result<(), LiveDataProcessorFailure>;
}

impl ParseEvents for Server {
  /// ## Rules for parsing messages
  /// * SpellCast, DamageDone, HealDone, Threat comes in any order, but
  ///   * Roughly at the same time +- 50ms (or even less)
  /// * Instance starts of Arena and Battleground
  ///   * The start is only called when it really starts, e.g. there are events up to 1 min
  ///     that need to be assigned to that instance then
  ///   * The end of an arena or battleground match is the end
  ///     * But some events after (~1s), still have to be assigned to that instance
  ///     * Every event after can be safely assumed to be in a new instance for Arena/BG
  /// * Map information is provided through the position of a player, hence we will not always
  ///   know immediately where to assign the player to.
  /// * For Raids, we have to distinguish between a continuation and a new instance
  ///   * InstanceIds are recycled upon every reset (There is a data structure that
  ///     provides information about raid reset times)
  ///   * Events after this reset have to be assigned to a completely new raid Id
  ///     * Even if it is the exact same guild.
  ///     * TODO: How about raid Id expansion? => There is also a table that could help
  /// * Interrupts are events that the spell cast was interrupted due to SOME reason
  ///   * One of the reasons may be an interrupt spell
  ///   * Other reasons are just running etc. (TODO: Use this information for the replay system)
  /// * Summon events can also come in theory after the npc already did something
  ///   * Not necessarily attacks, but buffs and stuffs (TODO: Test)
  fn parse_events(&mut self, mut messages: Vec<Message>) -> Result<(), LiveDataProcessorFailure> {
    self.non_committed_messages.append(&mut messages);

    // TODO: Think about a timeout, like NoOp events
    while let Some(mut event) = extract_committable_event(&mut self.non_committed_messages) {
      event.id = (self.committed_events.len() + 1) as u32;
      self.committed_events.push(event);
    }
    Ok(())
  }
}

fn extract_committable_event(non_committed_messages: &mut Vec<Message>) -> Option<Event> {
  if non_committed_messages.is_empty() {
    return None;
  }

  let first_message = non_committed_messages.first().expect("Should exist!").clone();
  match first_message.message_type {
    // Spell
    MessageType::SpellCast(_) |
    MessageType::Threat(_) |
    MessageType::Heal(_) |
    MessageType::MeleeDamage(_) |
    MessageType::SpellDamage(_) => try_parse_spell_cast(non_committed_messages, &first_message),

    // Instance stuff
    MessageType::InstancePvPStart(_) |
    MessageType::InstancePvPEndUnratedArena(_) |
    MessageType::InstancePvPEndBattleground(_) |
    MessageType::InstancePvPEndRatedArena(_) |

    // Can be safely committed, once we know the context
    MessageType::Death(_) |
    MessageType::CombatState(_) |
    MessageType::Loot(_) |
    MessageType::Summon(_) |
    MessageType::Power(_) |
    MessageType::Position(_) |
    MessageType::Event(_) |

    // Requires an existing spell
    MessageType::Interrupt(_) |
    MessageType::SpellSteal(_) |
    MessageType::Dispel(_) |
    MessageType::AuraApplication(_) => {
      non_committed_messages.pop().expect("Just remove it for now!"); // TODO
      None
    }
  }
}


/*
 * TRY PARSE SPELL START
 */

/// ## What has to be done here?
/// * We have to collect a spell cast
///   * This can have optional heal, damage done and threat
/// * If we have a spell cast but none other and the timeframe runs out,
///   then we will discard potential other and commit the spell cast
/// * NOTE: Melee damage does not have casts, but we will treat it as
///   a spell internally anyway
fn try_parse_spell_cast(non_committed_messages: &mut Vec<Message>, first_message: &Message) -> Option<Event> {
  let mut spell_cast_message = None;
  let mut spell_damage_done_message = None;
  let mut melee_damage_done_message = None;
  let mut threat_message = None;
  let mut heal_message = None;
  let mut reached_timeout = false;
  for msg in non_committed_messages.iter() {
    if msg.timestamp - first_message.timestamp > 50 {
      reached_timeout = true;
      break;
    } else if !spell_matches(&first_message, &msg) {
      continue;
    }

    match msg.message_type {
      MessageType::SpellCast(_) => spell_cast_message = Some(msg.clone()),
      MessageType::SpellDamage(_) => spell_damage_done_message = Some(msg.clone()),
      MessageType::MeleeDamage(_) => melee_damage_done_message = Some(msg.clone()),
      MessageType::Threat(_) => threat_message = Some(msg.clone()),
      MessageType::Heal(_) => heal_message = Some(msg.clone()),
      _ => continue
    }
  }

  if let Some(melee_damage_done_message) = melee_damage_done_message {
    if let MessageType::MeleeDamage(melee_damage) = &melee_damage_done_message.message_type {
      if reached_timeout || threat_message.is_some() {
        non_committed_messages.remove_item(&melee_damage_done_message).expect("Should be deleted!");
        let mut mitigation = Vec::new();
        if melee_damage.blocked > 0 { mitigation.push(Mitigation::Block(melee_damage.blocked)); }
        if melee_damage.absorbed > 0 { mitigation.push(Mitigation::Absorb(melee_damage.absorbed)); }
        if melee_damage.resisted_or_glanced > 0 { mitigation.push(Mitigation::Glance(melee_damage.resisted_or_glanced)); }

        return Some(Event {
          id: 0,
          timestamp: first_message.timestamp,
          subject: extract_spell_attacker(&first_message.message_type).expect("For spells this is always some").to_unit().expect("Must be an Unit"), // TODO: We need to handle this!
          event: EventType::SpellCast(SpellCast {
            victim: melee_damage.victim.to_unit().ok(),
            hit_type: HitType::Evade, // TODO
            spell_id: None,
            damage: Some(Damage {
              school: School::from_u8(melee_damage.school),
              damage: melee_damage.damage,
              mitigation
            }),
            heal: None,
            threat: threat_message.and_then(|message| {
              if let MessageType::Threat(threat) = &message.message_type {
                non_committed_messages.remove_item(&message).expect("Should be deleted!");
                Some(threat.amount)
              } else { None }
            })
          })
        });
      }
      return None;
    }
  } else if let Some(spell_cast_message) = spell_cast_message {
    if let MessageType::SpellCast(spell_cast) = &spell_cast_message.message_type {
      if reached_timeout || (threat_message.is_some() && (spell_damage_done_message.is_some() || heal_message.is_some())) {
        non_committed_messages.remove_item(&spell_cast_message).expect("Should be deleted!");
        return Some(Event {
          id: 0,
          timestamp: first_message.timestamp,
          subject: extract_spell_attacker(&first_message.message_type).expect("For spells this is always some").to_unit().expect("Must be an Unit"), // TODO: We need to handle this!
          event: EventType::SpellCast(SpellCast {
            victim: spell_cast.target.map(|target| target.to_unit().expect("Must be an Unit")), // TODO: Handle this
            hit_type: HitType::from_u8(spell_cast.hit_type), // TODO: CHeck if this matches!
            spell_id: Some(spell_cast.spell_id),
            damage: spell_damage_done_message.and_then(|message| {
              // Always true
              if let MessageType::SpellDamage(spell_damage) = &message.message_type {
                non_committed_messages.remove_item(&message).expect("Should be deleted!");
                let mut mitigation = Vec::new();
                if spell_damage.blocked > 0 { mitigation.push(Mitigation::Block(spell_damage.blocked)); }
                if spell_damage.absorbed > 0 { mitigation.push(Mitigation::Absorb(spell_damage.absorbed)); }
                if spell_damage.resisted_or_glanced > 0 { mitigation.push(Mitigation::Resist(spell_damage.resisted_or_glanced)); }
                return Some(Damage {
                  school: School::from_u8(spell_damage.school),
                  damage: spell_damage.damage,
                  mitigation
                });
              }
              None
            }),
            heal: heal_message.and_then(|message| {
              // Always true
              if let MessageType::Heal(heal) = &message.message_type {
                non_committed_messages.remove_item(&message).expect("Should be deleted!");
                return Some(Heal {
                  total: heal.total_heal,
                  effective: heal.effective_heal,
                  mitigation: if heal.absorb > 0 { vec![Mitigation::Absorb(heal.absorb)] } else { Vec::new() }
                })
              }
              None
            }),
            threat: threat_message.and_then(|message| {
              if let MessageType::Threat(threat) = &message.message_type {
                non_committed_messages.remove_item(&message).expect("Should be deleted!");
                Some(threat.amount)
              } else { None }
            })
          })
        });
      }
      return None;
    }
  }

  // Timeout, remove these messages then!
  if reached_timeout {
    if let Some(message) = threat_message { non_committed_messages.remove_item(&message).expect("Should be deleted!"); }
    if let Some(message) = spell_damage_done_message { non_committed_messages.remove_item(&message).expect("Should be deleted!"); }
    if let Some(message) = heal_message { non_committed_messages.remove_item(&message).expect("Should be deleted!"); }
  }
  None
}

fn extract_spell_attacker(message_type: &MessageType) -> Option<u64> {
  match message_type {
    MessageType::MeleeDamage(item) => Some(item.attacker),
    MessageType::SpellDamage(item) => Some(item.attacker),
    MessageType::Heal(item) => Some(item.caster),
    MessageType::SpellCast(item) => Some(item.caster),
    MessageType::Threat(item) => Some(item.threater),
    _ => None
  }
}
fn extract_spell_target(message_type: &MessageType) -> Option<u64> {
  match message_type {
    MessageType::MeleeDamage(item) => Some(item.victim),
    MessageType::SpellDamage(item) => Some(item.victim),
    MessageType::Heal(item) => Some(item.target),
    MessageType::SpellCast(item) => item.target,
    MessageType::Threat(item) => Some(item.threatened),
    _ => None
  }
}
fn extract_spell_id(message_type: &MessageType) -> Option<u32> {
  match message_type {
    MessageType::MeleeDamage(item) => item.spell_id,
    MessageType::SpellDamage(item) => item.spell_id,
    MessageType::Heal(item) => Some(item.spell_id),
    MessageType::SpellCast(item) => Some(item.spell_id),
    MessageType::Threat(item) => item.spell_id,
    _ => None
  }
}
fn spell_matches(msg1: &Message, msg2: &Message) -> bool {
  extract_spell_attacker(&msg2.message_type) == extract_spell_attacker(&msg1.message_type)
    && extract_spell_target(&msg2.message_type) == extract_spell_target(&msg1.message_type)
    && extract_spell_id(&msg2.message_type) == extract_spell_id(&msg1.message_type)
}

/*
 * TRY PARSE SPELL END
 */



fn extract_subject(message_type: &MessageType) -> u64 {
  match message_type {
    MessageType::MeleeDamage(item) => item.attacker,
    MessageType::SpellDamage(item) => item.attacker,
    MessageType::Heal(item) => item.caster,
    MessageType::Death(item) => item.victim, // ?
    MessageType::AuraApplication(item) => item.target, // ?
    MessageType::Dispel(item) => item.un_aura_caster,
    MessageType::SpellSteal(item) => item.un_aura_caster,
    MessageType::Position(item) => item.unit,
    MessageType::CombatState(item) => item.unit,
    MessageType::Power(item) => item.unit,
    MessageType::Loot(item) => item.unit,
    MessageType::SpellCast(item) => item.caster,
    MessageType::Threat(item) => item.threater,
    MessageType::Event(item) => item.unit,
    MessageType::Summon(item) => item.unit, // ?

    // TODO!
    MessageType::Interrupt(_) |
    MessageType::InstancePvPStart(_) |
    MessageType::InstancePvPEndUnratedArena(_) |
    MessageType::InstancePvPEndRatedArena(_) |
    MessageType::InstancePvPEndBattleground(_) => 0,
  }
}