use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Position};

pub trait MapPosition {
  fn to_position(&self) -> Result<Position, LiveDataProcessorFailure>;
}

impl MapPosition for [u8] {
  fn to_position(&self) -> Result<Position, LiveDataProcessorFailure> {
    unimplemented!()
  }
}