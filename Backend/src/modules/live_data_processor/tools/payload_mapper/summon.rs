use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Summon};

pub trait MapSummon {
  fn to_summon(&self) -> Result<Summon, LiveDataProcessorFailure>;
}

impl MapSummon for [u8] {
  fn to_summon(&self) -> Result<Summon, LiveDataProcessorFailure> {
    unimplemented!()
  }
}