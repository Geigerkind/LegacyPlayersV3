use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, HealDone};

pub trait MapHealDone {
  fn to_heal_done(&self) -> Result<HealDone, LiveDataProcessorFailure>;
}

impl MapHealDone for [u8] {
  fn to_heal_done(&self) -> Result<HealDone, LiveDataProcessorFailure> {
    unimplemented!()
  }
}