use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Event};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapEvent {
  fn to_event(&self) -> Result<Event, LiveDataProcessorFailure>;
}

impl MapEvent for [u8] {
  fn to_event(&self) -> Result<Event, LiveDataProcessorFailure> {
    if self.len() != 8 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(Event {
      unit: byte_reader::read_u64(&self[0..8])?,
      event_type: self[8]
    })
  }
}