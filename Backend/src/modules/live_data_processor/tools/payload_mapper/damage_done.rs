use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, DamageDone};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapDamageDone {
  fn from_melee_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure>;
  fn from_spell_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure>;
}

impl MapDamageDone for [u8] {
  fn from_melee_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure> {
    if self.len() != 33 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(DamageDone {
      attacker: byte_reader::read_u64(&self[0..8])?,
      victim: byte_reader::read_u64(&self[8..16])?,
      spell_id: None,
      blocked: byte_reader::read_u32(&self[16..20])?,
      school: self[20],
      damage: byte_reader::read_u32(&self[21..25])?,
      resisted_or_glanced: byte_reader::read_u32(&self[25..29])?,
      absorbed: byte_reader::read_u32(&self[29..33])?
    })
  }

  fn from_spell_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure> {
    if self.len() != 37 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(DamageDone {
      attacker: byte_reader::read_u64(&self[0..8])?,
      victim: byte_reader::read_u64(&self[8..16])?,
      spell_id: Some(byte_reader::read_u32(&self[16..20])?),
      blocked: byte_reader::read_u32(&self[20..24])?,
      school: self[24],
      damage: byte_reader::read_u32(&self[24..29])?,
      resisted_or_glanced: byte_reader::read_u32(&self[29..33])?,
      absorbed: byte_reader::read_u32(&self[33..37])?
    })
  }
}