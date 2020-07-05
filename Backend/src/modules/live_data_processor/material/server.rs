use crate::modules::live_data_processor::domain_value::{Event, NonCommittedEvent};
use std::collections::HashMap;

pub struct Server {
    pub server_id: u32,
    pub summons: HashMap<u64, u64>,                            // TODO: This grows uncontrollable
    pub non_committed_events: HashMap<u64, NonCommittedEvent>, // Mapping player to non committed event
    pub committed_events: Vec<Event>,                          // TODO: Write into files instead
}

impl Server {
    pub fn new(server_id: u32) -> Self {
        Server {
            server_id,
            summons: HashMap::new(),
            non_committed_events: HashMap::new(),
            committed_events: Vec::new(),
        }
    }
}
