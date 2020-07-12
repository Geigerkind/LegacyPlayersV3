use crate::modules::live_data_processor::material::Server;
use std::collections::HashMap;
use std::sync::RwLock;
use crate::util::database::Select;

pub struct LiveDataProcessor {
    pub servers: HashMap<u32, RwLock<Server>>,
}

impl Default for LiveDataProcessor {
    fn default() -> Self {
        LiveDataProcessor { servers: HashMap::new() }
    }
}

impl LiveDataProcessor {
    pub fn init(mut self, db_main: &mut impl Select) -> Self {
        // TODO: Get real amount of servers
        for i in 0..3 {
            self.servers.insert(i, RwLock::new(Server::new(i).init(db_main)));
        }

        self
    }
}
