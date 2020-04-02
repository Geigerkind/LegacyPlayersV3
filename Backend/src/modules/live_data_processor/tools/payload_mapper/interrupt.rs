use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Interrupt};

pub trait MapInterrupt {
  fn to_interrupt(&self) -> Result<Interrupt, LiveDataProcessorFailure>;
}

impl MapInterrupt for [u8] {
  fn to_interrupt(&self) -> Result<Interrupt, LiveDataProcessorFailure> {
    unimplemented!()
  }
}