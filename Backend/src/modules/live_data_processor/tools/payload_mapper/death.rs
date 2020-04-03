use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Death};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapDeath {
  fn to_death(&self) -> Result<Death, LiveDataProcessorFailure>;
}

impl MapDeath for [u8] {
  fn to_death(&self) -> Result<Death, LiveDataProcessorFailure> {
    Ok(Death {
      cause: byte_reader::read_u64(&self[0..8])?,
      victim: byte_reader::read_u64(&self[8..16])?
    })
  }
}