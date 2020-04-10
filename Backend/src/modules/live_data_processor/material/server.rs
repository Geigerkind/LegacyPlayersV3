use crate::modules::live_data_processor::dto::Message;
use crate::modules::live_data_processor::domain_value::Event;
use std::collections::HashMap;

pub struct Server {
  pub summons: HashMap<u64, u64>, // TODO: This grows uncontrollable
  pub non_committed_messages: Vec<Message>, // TODO: Smarter Data structure that splits per unit
  pub committed_events: Vec<Event> // TODO: Write into files instead
}

impl Default for Server {
  fn default() -> Self {
    Server {
      summons: HashMap::new(),
      non_committed_messages: Vec::new(),
      committed_events: Vec::new()
    }
  }
}

impl Server {
  pub fn init(self) -> Self {
    self
  }
}