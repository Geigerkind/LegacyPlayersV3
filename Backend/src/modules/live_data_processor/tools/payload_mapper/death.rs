use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Death};

pub trait MapDeath {
  fn to_death(&self) -> Result<Death, LiveDataProcessorFailure>;
}

impl MapDeath for [u8] {
  fn to_death(&self) -> Result<Death, LiveDataProcessorFailure> {
    unimplemented!()
  }
}