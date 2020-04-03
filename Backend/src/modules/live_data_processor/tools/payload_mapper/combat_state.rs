use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, CombatState};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapCombatState {
  fn to_combat_state(&self) -> Result<CombatState, LiveDataProcessorFailure>;
}

impl MapCombatState for [u8] {
  fn to_combat_state(&self) -> Result<CombatState, LiveDataProcessorFailure> {
    if self.len() != 8 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(CombatState {
      unit: byte_reader::read_u64(&self[0..8])?,
      in_combat: if self[8] == 1 { true } else { false }
    })
  }
}