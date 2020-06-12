use crate::modules::live_data_processor::dto::{Unit, LiveDataProcessorFailure};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapUnit {
  fn to_unit(&self) -> Result<Unit, LiveDataProcessorFailure>;
}

impl MapUnit for [u8] {
  fn to_unit(&self) -> Result<Unit, LiveDataProcessorFailure> {
    if self.len() != 9 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(Unit {
      is_player: self[0] == 1,
      unit_id: byte_reader::read_u64(&self[1..9]).unwrap()
    })
  }
}