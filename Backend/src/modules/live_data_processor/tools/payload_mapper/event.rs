use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Event};

pub trait MapEvent {
  fn to_event(&self) -> Result<Event, LiveDataProcessorFailure>;
}

impl MapEvent for [u8] {
  fn to_event(&self) -> Result<Event, LiveDataProcessorFailure> {
    unimplemented!()
  }
}