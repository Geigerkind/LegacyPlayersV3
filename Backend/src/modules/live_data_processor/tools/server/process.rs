use crate::modules::live_data_processor::dto::{Message, LiveDataProcessorFailure, MessageType};
use crate::modules::live_data_processor::material::Server;
use crate::modules::live_data_processor::domain_value::{SpellCast, Event, EventType, HitType, Damage, School, Mitigation, Heal};
use crate::modules::live_data_processor::tools::MapUnit;
use crate::modules::live_data_processor::tools::server::try_parse_spell_cast;

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
  ///     * TODO: How about raids that have been started but reset before the first boss?
  /// * Interrupts are events that the spell cast was interrupted due to SOME reason
  ///   * One of the reasons may be an interrupt spell
  ///   * Other reasons are just running etc. (TODO: Use this information for the replay system)
  /// * Summon events can also come in theory after the npc already did something
  ///   * Not necessarily attacks, but buffs and stuffs (TODO: Test)
  /// NOTE: We assume here a stream of messages, so for a server that stops sending messages
  ///       Part of them may not be fully processed.
  fn parse_events(&mut self, mut messages: Vec<Message>) -> Result<(), LiveDataProcessorFailure> {
    self.non_committed_messages.append(&mut messages);

    while let Some(mut event) = extract_committable_event(&mut self.non_committed_messages) {
      event.id = (self.committed_events.len() + 1) as u32;
      println!("Parsed event: {:?}", event);
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
  let subject;
  if let Some(unit_id) = extract_subject(&first_message.message_type) {
    if let Ok(attacker) = unit_id.to_unit() {
      subject = attacker;
    } else {
      non_committed_messages.remove_item(&first_message);
      return None;
    }
  } else {
    non_committed_messages.remove_item(&first_message);
    return None;
  }

  let mut event = Event {
    id: 0,
    timestamp: first_message.timestamp,
    subject,
    event: EventType::PlaceHolder
  };

  match first_message.message_type {
    // Spell
    MessageType::SpellCast(_) |
    MessageType::Threat(_) |
    MessageType::Heal(_) |
    MessageType::MeleeDamage(_) |
    MessageType::SpellDamage(_) => event.event = EventType::SpellCast(try_parse_spell_cast(non_committed_messages, &first_message)?),

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
      return None;
    }
  };
  return Some(event);
}

fn extract_subject(message_type: &MessageType) -> Option<u64> {
  match message_type {
    MessageType::MeleeDamage(item) => Some(item.attacker),
    MessageType::SpellDamage(item) => Some(item.attacker),
    MessageType::Heal(item) => Some(item.caster),
    MessageType::Death(item) => Some(item.victim), // ?
    MessageType::AuraApplication(item) => Some(item.target), // ?
    MessageType::Dispel(item) => Some(item.un_aura_caster),
    MessageType::SpellSteal(item) => Some(item.un_aura_caster),
    MessageType::Position(item) => Some(item.unit),
    MessageType::CombatState(item) => Some(item.unit),
    MessageType::Power(item) => Some(item.unit),
    MessageType::Loot(item) => Some(item.unit),
    MessageType::SpellCast(item) => Some(item.caster),
    MessageType::Threat(item) => Some(item.threater),
    MessageType::Event(item) => Some(item.unit),
    MessageType::Summon(item) => Some(item.unit), // ?

    // TODO!
    MessageType::Interrupt(_) |
    MessageType::InstancePvPStart(_) |
    MessageType::InstancePvPEndUnratedArena(_) |
    MessageType::InstancePvPEndRatedArena(_) |
    MessageType::InstancePvPEndBattleground(_) => None,
  }
}