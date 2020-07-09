use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Summon};
use crate::modules::live_data_processor::tools::payload_mapper::unit::MapUnit;

pub trait MapSummon {
    fn to_summon(&self) -> Result<Summon, LiveDataProcessorFailure>;
}

impl MapSummon for [u8] {
    fn to_summon(&self) -> Result<Summon, LiveDataProcessorFailure> {
        if self.len() != 16 {
            return Err(LiveDataProcessorFailure::InvalidInput);
        }
        Ok(Summon {
            owner: self[0..8].to_unit()?,
            unit: self[8..16].to_unit()?,
        })
    }
}
