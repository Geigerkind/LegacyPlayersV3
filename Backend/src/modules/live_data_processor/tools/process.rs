use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Message};
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::modules::live_data_processor::domain_value::Event;
use crate::modules::live_data_processor::tools::{MessageParser, EventParser};

pub trait ProcessMessages {
  fn process_messages(&self, server_id: u32, messages: Vec<String>) -> Result<(), LiveDataProcessorFailure>;
}

impl ProcessMessages for LiveDataProcessor {
  fn process_messages(&self, server_id: u32, messages: Vec<String>) -> Result<(), LiveDataProcessorFailure> {
    let events = messages
      .iter()
      .map(|msg| msg.parse_message())
      .filter(|res| res.is_ok())
      .map(|msg_res| msg_res.unwrap())
      .collect::<Vec<Message>>()
      .to_event_vec()?;

    unimplemented!()
  }
}