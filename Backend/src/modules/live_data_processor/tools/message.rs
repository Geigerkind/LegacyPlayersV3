use crate::modules::live_data_processor::dto::{Message, LiveDataProcessorFailure};
use crate::modules::live_data_processor::LiveDataProcessor;

pub trait MessageParser {
  fn parse_message(&self) -> Result<Message, LiveDataProcessorFailure>;
}

impl MessageParser for String {
  fn parse_message(&self) -> Result<Message, LiveDataProcessorFailure> {
    unimplemented!()
  }
}