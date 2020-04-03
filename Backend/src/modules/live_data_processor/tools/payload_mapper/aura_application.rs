use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, AuraApplication};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapAuraApplication {
  fn to_aura_application(&self) -> Result<AuraApplication, LiveDataProcessorFailure>;
}

impl MapAuraApplication for [u8] {
  fn to_aura_application(&self) -> Result<AuraApplication, LiveDataProcessorFailure> {
    if self.len() != 21 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(AuraApplication {
      caster: byte_reader::read_u64(&self[0..8])?,
      target: byte_reader::read_u64(&self[8..16])?,
      spell_id: byte_reader::read_u32(&self[16..20])?,
      stack_amount: self[20],
      applied: if self[21] == 1 { true } else { false }
    })
  }
}