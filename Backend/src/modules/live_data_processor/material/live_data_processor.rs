use crate::modules::live_data_processor::material::Server;
use std::sync::RwLock;
use std::collections::HashMap;

pub struct LiveDataProcessor {
  pub servers: HashMap<u32, RwLock<Server>>
}

impl Default for LiveDataProcessor {
  fn default() -> Self {
    LiveDataProcessor {
      servers: HashMap::new()
    }
  }
}

impl LiveDataProcessor {
  pub fn init(mut self) -> Self {
    self
  }
}