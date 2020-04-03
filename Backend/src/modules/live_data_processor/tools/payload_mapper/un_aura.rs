use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, UnAura};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapUnAura {
  fn to_un_aura(&self) -> Result<UnAura, LiveDataProcessorFailure>;
}

impl MapUnAura for [u8] {
  fn to_un_aura(&self) -> Result<UnAura, LiveDataProcessorFailure> {
    if self.len() != 32 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(UnAura {
      un_aura_caster: byte_reader::read_u64(&self[0..8])?,
      target: byte_reader::read_u64(&self[8..16])?,
      aura_caster: byte_reader::read_u64(&self[16..24])?,
      un_aura_spell_id: byte_reader::read_u32(&self[24..28])?,
      target_spell_id: byte_reader::read_u32(&self[28..32])?,
      un_aura_amount: self[32]
    })
  }
}