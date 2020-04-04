use crate::modules::live_data_processor::dto::{Message, LiveDataProcessorFailure};
use crate::modules::live_data_processor::material::Server;

pub trait ParseEvents {
  fn parse_events(&mut self, messages: Vec<Message>) -> Result<(), LiveDataProcessorFailure>;
}

impl ParseEvents for Server {
  fn parse_events(&mut self, messages: Vec<Message>) -> Result<(), LiveDataProcessorFailure> {
    unimplemented!()
  }
}