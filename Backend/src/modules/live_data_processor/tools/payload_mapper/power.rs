use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Power};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapPower {
  fn to_power(&self) -> Result<Power, LiveDataProcessorFailure>;
}

impl MapPower for [u8] {
  fn to_power(&self) -> Result<Power, LiveDataProcessorFailure> {
    Ok(Power {
      unit: byte_reader::read_u64(&self[0..8])?,
      power_type: self[8],
      max_power: byte_reader::read_u32(&self[9..13])?,
      current_power: byte_reader::read_u32(&self[13..17])?
    })
  }
}