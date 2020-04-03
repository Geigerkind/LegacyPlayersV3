use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Summon};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapSummon {
  fn to_summon(&self) -> Result<Summon, LiveDataProcessorFailure>;
}

impl MapSummon for [u8] {
  fn to_summon(&self) -> Result<Summon, LiveDataProcessorFailure> {
    Ok(Summon {
      owner: byte_reader::read_u64(&self[0..8])?,
      unit: byte_reader::read_u64(&self[8..16])?
    })
  }
}