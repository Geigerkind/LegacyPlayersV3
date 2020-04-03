use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, SpellCast};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapSpellCast {
  fn to_spell_cast(&self) -> Result<SpellCast, LiveDataProcessorFailure>;
}

impl MapSpellCast for [u8] {
  fn to_spell_cast(&self) -> Result<SpellCast, LiveDataProcessorFailure> {
    let target_id = byte_reader::read_u64(&self[8..16])?;
    Ok(SpellCast {
      caster: byte_reader::read_u64(&self[0..8])?,
      target: if target_id == 0 { None } else { Some(target_id) },
      spell_id: byte_reader::read_u32(&self[16..20])?,
      hit_type: self[20]
    })
  }
}