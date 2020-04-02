use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Threat};

pub trait MapThreat {
  fn to_threat(&self) -> Result<Threat, LiveDataProcessorFailure>;
}

impl MapThreat for [u8] {
  fn to_threat(&self) -> Result<Threat, LiveDataProcessorFailure> {
    unimplemented!()
  }
}