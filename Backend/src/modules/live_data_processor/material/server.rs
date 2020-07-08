use crate::modules::live_data_processor::domain_value::{Event, NonCommittedEvent, UnitInstance};
use std::collections::HashMap;

pub struct Server {
    pub server_id: u32,

    // Meta Data
    pub summons: HashMap<u64, u64>,                            // TODO: This grows uncontrollable
    pub active_instances: HashMap<u32, UnitInstance>,
    pub unit_instance_id: HashMap<u64, u32>,

    // Events
    pub non_committed_events: HashMap<u64, NonCommittedEvent>, // Mapping player to non committed event
    pub committed_events: Vec<Event>,                          // TODO: Write into files instead
}

impl Server {
    pub fn new(server_id: u32) -> Self {
        Server {
            server_id,
            summons: HashMap::new(),
            active_instances: HashMap::new(),
            unit_instance_id: HashMap::new(),
            non_committed_events: HashMap::new(),
            committed_events: Vec::new(),
        }
    }
}
