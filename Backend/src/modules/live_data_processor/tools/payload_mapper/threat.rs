use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Threat};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapThreat {
  fn to_threat(&self) -> Result<Threat, LiveDataProcessorFailure>;
}

impl MapThreat for [u8] {
  fn to_threat(&self) -> Result<Threat, LiveDataProcessorFailure> {
    Ok(Threat {
      threater: byte_reader::read_u64(&self[0..8])?,
      threatened: byte_reader::read_u64(&self[8..16])?,
      spell_id: byte_reader::read_u32(&self[16..20])?,
      amount: byte_reader::read_i32(&self[20..24])?
    })
  }
}