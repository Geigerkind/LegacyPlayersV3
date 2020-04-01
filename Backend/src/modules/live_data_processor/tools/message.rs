use crate::modules::live_data_processor::dto::{Message, ParseMessageType, LiveDataProcessorFailure};
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MessageParser {
  fn parse_message(&self) -> Result<Message, LiveDataProcessorFailure>;
}

impl MessageParser for String {
  fn parse_message(&self) -> Result<Message, LiveDataProcessorFailure> {
    let bytes = self.as_bytes();
    if bytes.len() <= 9 {
      return Err(LiveDataProcessorFailure::InvalidInput);
    }

    let api_version = bytes[0];
    let message_type = bytes[1].to_message_type()?;
    let message_length = bytes[2];
    let timestamp = byte_reader::read_u64(&bytes[3..7]);
    let payload = bytes[7..bytes.len()].to_vec();

    Ok(Message {
      api_version, message_type, message_length, timestamp, payload
    })
  }
}