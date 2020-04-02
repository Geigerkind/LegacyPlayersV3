use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Power};

pub trait MapPower {
  fn to_power(&self) -> Result<Power, LiveDataProcessorFailure>;
}

impl MapPower for [u8] {
  fn to_power(&self) -> Result<Power, LiveDataProcessorFailure> {
    unimplemented!()
  }
}