use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Instance};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapInstance {
  fn to_instance(&self) -> Result<Instance, LiveDataProcessorFailure>;
}

impl MapInstance for [u8] {
  fn to_instance(&self) -> Result<Instance, LiveDataProcessorFailure> {
    if self.len() != 8 && self.len() != 9 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(Instance {
      map_id: byte_reader::read_u32(&self[0..4])?,
      instance_id: byte_reader::read_u32(&self[4..8])?,
      winner: self.get(8).cloned()
    })
  }
}
