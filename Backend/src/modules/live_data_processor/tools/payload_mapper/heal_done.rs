use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, HealDone};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapHealDone {
  fn to_heal_done(&self) -> Result<HealDone, LiveDataProcessorFailure>;
}

impl MapHealDone for [u8] {
  fn to_heal_done(&self) -> Result<HealDone, LiveDataProcessorFailure> {
    if self.len() != 32 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(HealDone {
      caster: byte_reader::read_u64(&self[0..8])?,
      target: byte_reader::read_u64(&self[8..16])?,
      spell_id: byte_reader::read_u32(&self[16..20])?,
      total_heal: byte_reader::read_u32(&self[20..24])?,
      effective_heal: byte_reader::read_u32(&self[24..28])?,
      absorb: byte_reader::read_u32(&self[28..32])?
    })
  }
}