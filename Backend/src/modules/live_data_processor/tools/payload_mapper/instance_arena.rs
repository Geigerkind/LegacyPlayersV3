use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, InstanceArena};

pub trait MapInstanceArena {
  fn to_instance_arena(&self) -> Result<InstanceArena, LiveDataProcessorFailure>;
}

impl MapInstanceArena for [u8] {
  fn to_instance_arena(&self) -> Result<InstanceArena, LiveDataProcessorFailure> {
    unimplemented!()
  }
}