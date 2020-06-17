use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Position};
use crate::modules::live_data_processor::tools::byte_reader;
use crate::modules::live_data_processor::tools::payload_mapper::unit::MapUnit;

pub trait MapPosition {
    fn to_position(&self) -> Result<Position, LiveDataProcessorFailure>;
}

impl MapPosition for [u8] {
    fn to_position(&self) -> Result<Position, LiveDataProcessorFailure> {
        if self.len() != 34 {
            return Err(LiveDataProcessorFailure::InvalidInput);
        }
        Ok(Position {
            map_id: byte_reader::read_u32(&self[0..4])?,
            instance_id: byte_reader::read_u32(&self[4..8])?,
            map_difficulty: self[8],
            unit: self[9..18].to_unit()?,
            x: byte_reader::read_i32(&self[18..22])?,
            y: byte_reader::read_i32(&self[22..26])?,
            z: byte_reader::read_i32(&self[26..30])?,
            orientation: byte_reader::read_i32(&self[30..34])?,
        })
    }
}
