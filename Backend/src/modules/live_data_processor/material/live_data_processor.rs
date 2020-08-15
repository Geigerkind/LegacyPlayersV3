use crate::modules::live_data_processor::material::Server;
use crate::util::database::Select;
use std::collections::HashMap;
use std::sync::RwLock;

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
        db_main.select("SELECT id, expansion_id FROM data_server",
            |mut row| (row.take::<u32, usize>(0).unwrap(), row.take::<u8, usize>(1).unwrap()))
            .into_iter()
            .for_each(|(server_id, expansion_id)| {
                self.servers.insert(server_id, RwLock::new(Server::new(server_id, expansion_id).init(db_main)));
            });
        self
    }
}
