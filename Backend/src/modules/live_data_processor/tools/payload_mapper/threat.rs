use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Threat};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapThreat {
  fn to_threat(&self) -> Result<Threat, LiveDataProcessorFailure>;
}

impl MapThreat for [u8] {
  fn to_threat(&self) -> Result<Threat, LiveDataProcessorFailure> {
    if self.len() != 24 { return Err(LiveDataProcessorFailure::InvalidInput) }
    let spell_id = byte_reader::read_u32(&self[16..20])?;
    Ok(Threat {
      threater: byte_reader::read_u64(&self[0..8])?,
      threatened: byte_reader::read_u64(&self[8..16])?,
      spell_id: if spell_id == 0 { None } else { Some(spell_id) },
      amount: byte_reader::read_i32(&self[20..24])?
    })
  }
}