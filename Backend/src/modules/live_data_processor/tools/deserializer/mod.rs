use crate::modules::live_data_processor::domain_value::Unit;

pub mod unit;
pub mod damage;
pub mod event;

pub trait LiveDataDeserializer {
    fn deserialize(&self) -> String;
}

pub trait EventTypeDeserializer {
    fn deserialize(&self, subject: &Unit) -> String;
}