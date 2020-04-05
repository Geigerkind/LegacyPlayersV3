use crate::modules::live_data_processor::dto::{Message, LiveDataProcessorFailure};
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::modules::live_data_processor::tools::byte_reader;
use crate::modules::live_data_processor::tools::payload_mapper::MapMessageType;

pub trait MessageParser {
  fn parse_message(&self) -> Result<Message, LiveDataProcessorFailure>;
}

impl MessageParser for Vec<u8> {
  fn parse_message(&self) -> Result<Message, LiveDataProcessorFailure> {
    if self.len() <= 11 {
      return Err(LiveDataProcessorFailure::InvalidInput);
    }

    let api_version = self[0];
    let message_length = self[2];
    println!("Reading Timestamp of {:?}", &self[3..11]);
    let timestamp = byte_reader::read_u64(&self[3..11]).unwrap();
    println!("Timestamp: {}", timestamp);
    let payload = self[11..self.len()].to_vec();
    let message_type = self[1].to_message_type(&payload)?;

    Ok(Message {
      api_version,
      message_type,
      message_length,
      timestamp
    })
  }
}