use crate::modules::live_data_processor::dto::{MessageType, LiveDataProcessorFailure};
use crate::modules::live_data_processor::tools::payload_mapper::damage_done::MapDamageDone;
use crate::modules::live_data_processor::tools::payload_mapper::heal_done::MapHealDone;
use crate::modules::live_data_processor::tools::payload_mapper::death::MapDeath;
use crate::modules::live_data_processor::tools::payload_mapper::aura_application::MapAuraApplication;
use crate::modules::live_data_processor::tools::payload_mapper::un_aura::MapUnAura;
use crate::modules::live_data_processor::tools::payload_mapper::instance_battleground::MapInstanceBattleground;
use crate::modules::live_data_processor::tools::payload_mapper::instance_arena::MapInstanceArena;
use crate::modules::live_data_processor::tools::payload_mapper::instance::MapInstance;
use crate::modules::live_data_processor::tools::payload_mapper::summon::MapSummon;
use crate::modules::live_data_processor::tools::payload_mapper::event::MapEvent;
use crate::modules::live_data_processor::tools::payload_mapper::threat::MapThreat;
use crate::modules::live_data_processor::tools::payload_mapper::spell_cast::MapSpellCast;
use crate::modules::live_data_processor::tools::payload_mapper::loot::MapLoot;
use crate::modules::live_data_processor::tools::payload_mapper::power::MapPower;
use crate::modules::live_data_processor::tools::payload_mapper::combat_state::MapCombatState;
use crate::modules::live_data_processor::tools::payload_mapper::position::MapPosition;
use crate::modules::live_data_processor::tools::payload_mapper::interrupt::MapInterrupt;

pub trait MapMessageType {
  fn to_message_type(&self, payload: &[u8]) -> Result<MessageType, LiveDataProcessorFailure>;
}

impl MapMessageType for u8 {
  fn to_message_type(&self, payload: &[u8]) -> Result<MessageType, LiveDataProcessorFailure> {
    Ok(match self {
      0 => MessageType::MeleeDamage(payload.from_melee_damage()?),
      1 => MessageType::SpellDamage(payload.from_spell_damage()?),
      2 => MessageType::Heal(payload.to_heal_done()?),
      3 => MessageType::Death(payload.to_death()?),
      4 => MessageType::AuraApplication(payload.to_aura_application()?),
      5 => MessageType::Dispel(payload.to_un_aura()?),
      6 => MessageType::SpellSteal(payload.to_un_aura()?),
      7 => MessageType::Interrupt(payload.to_interrupt()?),
      8 => MessageType::Position(payload.to_position()?),
      9 => MessageType::CombatState(payload.to_combat_state()?),
      10 => MessageType::Power(payload.to_power()?),
      11 => MessageType::Loot(payload.to_loot()?),
      12 => MessageType::SpellCast(payload.to_spell_cast()?),
      13 => MessageType::Threat(payload.to_threat()?),
      14 => MessageType::Event(payload.to_event()?),
      15 => MessageType::Summon(payload.to_summon()?),
      16 => MessageType::InstancePvPStart(payload.to_instance()?),
      17 => MessageType::InstancePvPEndUnratedArena(payload.to_instance()?),
      18 => MessageType::InstancePvPEndRatedArena(payload.to_instance_arena()?),
      19 => MessageType::InstancePvPEndBattleground(payload.to_instance_battleground()?),
      _ => return Err(LiveDataProcessorFailure::InvalidInput)
    })
  }
}