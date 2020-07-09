use crate::modules::live_data_processor::dto::{DamageDone, LiveDataProcessorFailure};
use crate::modules::live_data_processor::tools::byte_reader;
use crate::modules::live_data_processor::tools::payload_mapper::unit::MapUnit;

pub trait MapDamageDone {
    fn from_melee_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure>;
    fn from_spell_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure>;
}

impl MapDamageDone for [u8] {
    fn from_melee_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure> {
        if self.len() != 34 {
            return Err(LiveDataProcessorFailure::InvalidInput);
        }
        Ok(DamageDone {
            attacker: self[0..8].to_unit()?,
            victim: self[8..16].to_unit()?,
            spell_id: None,
            blocked: byte_reader::read_u32(&self[16..20])?,
            hit_type: Some(self[20]),
            school: self[21],
            damage: byte_reader::read_u32(&self[22..26])?,
            resisted_or_glanced: byte_reader::read_u32(&self[26..30])?,
            absorbed: byte_reader::read_u32(&self[30..34])?,
        })
    }

    fn from_spell_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure> {
        if self.len() != 37 {
            return Err(LiveDataProcessorFailure::InvalidInput);
        }
        Ok(DamageDone {
            attacker: self[0..8].to_unit()?,
            victim: self[8..16].to_unit()?,
            spell_id: Some(byte_reader::read_u32(&self[16..20])?),
            hit_type: None,
            blocked: byte_reader::read_u32(&self[20..24])?,
            school: self[24],
            damage: byte_reader::read_u32(&self[25..29])?,
            resisted_or_glanced: byte_reader::read_u32(&self[29..33])?,
            absorbed: byte_reader::read_u32(&self[33..37])?,
        })
    }
}
