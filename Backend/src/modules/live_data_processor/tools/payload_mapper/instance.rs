use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Instance};

pub trait MapInstance {
  fn to_instance(&self) -> Result<Instance, LiveDataProcessorFailure>;
}

impl MapInstance for [u8] {
  fn to_instance(&self) -> Result<Instance, LiveDataProcessorFailure> {
    unimplemented!()
  }
}
