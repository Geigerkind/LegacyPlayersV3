use crate::modules::live_data_processor::dto::{DamageDone, LiveDataProcessorFailure};
use crate::modules::live_data_processor::tools::byte_reader;
use crate::modules::live_data_processor::tools::payload_mapper::unit::MapUnit;

pub trait MapDamageDone {
    fn from_melee_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure>;
    fn from_spell_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure>;
}

impl MapDamageDone for [u8] {
    fn from_melee_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure> {
        if self.len() != 39 {
            return Err(LiveDataProcessorFailure::InvalidInput);
        }
        Ok(DamageDone {
            attacker: self[0..9].to_unit()?,
            victim: self[9..18].to_unit()?,
            spell_id: None,
            blocked: byte_reader::read_u32(&self[18..22])?,
            hit_mask: byte_reader::read_u32(&self[22..26])?,
            school_mask: self[26],
            damage: byte_reader::read_u32(&self[27..31])?,
            resisted_or_glanced: byte_reader::read_u32(&self[31..35])?,
            absorbed: byte_reader::read_u32(&self[35..39])?,
            damage_over_time: false,
        })
    }

    fn from_spell_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure> {
        if self.len() != 44 {
            return Err(LiveDataProcessorFailure::InvalidInput);
        }
        Ok(DamageDone {
            attacker: self[0..9].to_unit()?,
            victim: self[9..18].to_unit()?,
            spell_id: Some(byte_reader::read_u32(&self[18..22])?),
            blocked: byte_reader::read_u32(&self[22..26])?,
            school_mask: self[26],
            damage: byte_reader::read_u32(&self[27..31])?,
            resisted_or_glanced: byte_reader::read_u32(&self[31..35])?,
            absorbed: byte_reader::read_u32(&self[35..39])?,
            damage_over_time: self[39] == 1,
            hit_mask: byte_reader::read_u32(&self[40..44])?,
        })
    }
}
