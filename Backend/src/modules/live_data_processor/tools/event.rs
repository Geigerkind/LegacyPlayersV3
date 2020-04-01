use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Message};
use crate::modules::live_data_processor::domain_value::Event;

pub trait EventParser {
  fn to_event_vec(self) -> Result<Vec<Event>, LiveDataProcessorFailure>;
}

impl EventParser for Vec<Message> {
  fn to_event_vec(self) -> Result<Vec<Event>, LiveDataProcessorFailure> {
    unimplemented!()
  }
}