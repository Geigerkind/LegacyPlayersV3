use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, DamageDone};

pub trait MapDamageDone {
  fn to_damage_done(&self) -> Result<DamageDone, LiveDataProcessorFailure>;
}

impl MapDamageDone for [u8] {
  fn to_damage_done(&self) -> Result<DamageDone, LiveDataProcessorFailure> {
    unimplemented!()
  }
}