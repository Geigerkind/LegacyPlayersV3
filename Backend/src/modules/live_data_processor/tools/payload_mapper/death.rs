use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Death};
use crate::modules::live_data_processor::tools::payload_mapper::unit::MapUnit;

pub trait MapDeath {
  fn to_death(&self) -> Result<Death, LiveDataProcessorFailure>;
}

impl MapDeath for [u8] {
  fn to_death(&self) -> Result<Death, LiveDataProcessorFailure> {
    if self.len() != 18 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(Death {
      cause: self[0..9].to_unit()?,
      victim: self[9..18].to_unit()?
    })
  }
}