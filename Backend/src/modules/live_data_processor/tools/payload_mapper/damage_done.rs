use crate::modules::live_data_processor::dto::{DamageDone, LiveDataProcessorFailure};
use crate::modules::live_data_processor::tools::byte_reader;
use crate::modules::live_data_processor::tools::payload_mapper::unit::MapUnit;

pub trait MapDamageDone {
    fn from_melee_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure>;
    fn from_spell_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure>;
}

impl MapDamageDone for [u8] {
    fn from_melee_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure> {
        if self.len() != 36 {
            return Err(LiveDataProcessorFailure::InvalidInput);
        }
        Ok(DamageDone {
            attacker: self[0..9].to_unit()?,
            victim: self[9..18].to_unit()?,
            spell_id: None,
            blocked: byte_reader::read_u32(&self[18..22])?,
            hit_type: Some(self[22]),
            school: self[23],
            damage: byte_reader::read_u32(&self[24..28])?,
            resisted_or_glanced: byte_reader::read_u32(&self[28..32])?,
            absorbed: byte_reader::read_u32(&self[32..36])?,
        })
    }

    fn from_spell_damage(&self) -> Result<DamageDone, LiveDataProcessorFailure> {
        if self.len() != 39 {
            return Err(LiveDataProcessorFailure::InvalidInput);
        }
        Ok(DamageDone {
            attacker: self[0..9].to_unit()?,
            victim: self[9..18].to_unit()?,
            spell_id: Some(byte_reader::read_u32(&self[18..22])?),
            hit_type: None,
            blocked: byte_reader::read_u32(&self[22..26])?,
            school: self[26],
            damage: byte_reader::read_u32(&self[27..31])?,
            resisted_or_glanced: byte_reader::read_u32(&self[31..35])?,
            absorbed: byte_reader::read_u32(&self[35..39])?,
        })
    }
}
