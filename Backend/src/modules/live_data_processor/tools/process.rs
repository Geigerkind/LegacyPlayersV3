use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Message};
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::modules::live_data_processor::domain_value::Event;
use crate::modules::live_data_processor::tools::MessageParser;
use crate::modules::live_data_processor::tools::server::ParseEvents;

pub trait ProcessMessages {
  fn process_messages(&self, server_id: u32, messages: Vec<Vec<u8>>) -> Result<(), LiveDataProcessorFailure>;
}

impl ProcessMessages for LiveDataProcessor {
  fn process_messages(&self, server_id: u32, messages: Vec<Vec<u8>>) -> Result<(), LiveDataProcessorFailure> {
    let msg_vec = messages
      .iter()
      .map(|msg| msg.parse_message())
      .filter(|res| res.is_ok())
      .map(|msg_res| msg_res.unwrap())
      .collect::<Vec<Message>>();

    if !msg_vec.is_empty() {
      let mut server = self.servers.get(&server_id).expect("Server Id must exist!").write().unwrap();
      return server.parse_events(msg_vec);
    }

    Ok(())
  }
}