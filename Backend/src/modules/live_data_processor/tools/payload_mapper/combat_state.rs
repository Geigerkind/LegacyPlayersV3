use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, CombatState};

pub trait MapCombatState {
  fn to_combat_state(&self) -> Result<CombatState, LiveDataProcessorFailure>;
}

impl MapCombatState for [u8] {
  fn to_combat_state(&self) -> Result<CombatState, LiveDataProcessorFailure> {
    unimplemented!()
  }
}