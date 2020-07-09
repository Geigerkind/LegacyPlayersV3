use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Message};
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
        println!("MSG TYPE {}, {:?}", self[1], self);
        println!("LENGTH: {}", self.len()-11);
        let message_type = self[1].to_message_type(&self[11..])?;
        println!("MSG {:?}", message_type);

        Ok(Message {
            api_version,
            message_type,
            message_length,
            timestamp,
        })
    }
}
