use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Unit};
use crate::modules::live_data_processor::tools::{byte_reader, GUID};

pub trait MapUnit {
    fn to_unit(&self) -> Result<Unit, LiveDataProcessorFailure>;
}

impl MapUnit for [u8] {
    fn to_unit(&self) -> Result<Unit, LiveDataProcessorFailure> {
        if self.len() != 8 {
            return Err(LiveDataProcessorFailure::InvalidInput);
        }
        let unit_id = byte_reader::read_u64(&self[0..8]).unwrap();
        Ok(Unit {
            is_player: unit_id.is_player(),
            unit_id,
        })
    }
}
