use crate::modules::live_data_processor::dto::{Message, LiveDataProcessorFailure, MessageType};
use crate::modules::live_data_processor::material::Server;
use crate::modules::live_data_processor::domain_value::{SpellCast, Event, EventType};
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
  fn parse_events(&mut self, messages: Vec<Message>) -> Result<(), LiveDataProcessorFailure> {
    for msg in messages {
      match msg.message_type {
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
        MessageType::Event(_) => {

        },

        // Combined events
        // Spell
        MessageType::SpellCast(_) |
        MessageType::Threat(_) |
        MessageType::Heal(_) |
        MessageType::MeleeDamage(_) |
        MessageType::SpellDamage(_) => {
          if let Some(spell_cast) = try_parse_spell() {
            self.committed_events.push(Event {
              id: (self.committed_events.len() + 1) as u32,
              timestamp: msg.timestamp,
              subject: extract_subject(&msg.message_type).to_unit()?,
              event: EventType::SpellCast(spell_cast)
            });
            continue;
          }

          self.non_committed_messages.push(msg);
          continue;
        },

        // Requires an existing spell
        MessageType::Interrupt(_) |
        MessageType::SpellSteal(_) |
        MessageType::Dispel(_) |
        MessageType::AuraApplication(_) => {

        }
      };
    }
    Ok(())
  }
}

fn try_parse_spell() -> Option<SpellCast> {

  unimplemented!()
}

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