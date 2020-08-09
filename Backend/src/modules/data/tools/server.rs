use crate::modules::data::material::Init;
use crate::modules::data::{dto::AvailableServer, Data};
use crate::util::database::Select;

pub trait RetrieveServer {
    fn get_server(&self, id: u32) -> Option<AvailableServer>;
    fn get_server_by_name(&self, server_name: String) -> Option<AvailableServer>;
    fn get_all_servers(&self) -> Vec<AvailableServer>;
    fn reload_server(&self, db_main: &mut impl Select);
}

impl RetrieveServer for Data {
    fn get_server(&self, id: u32) -> Option<AvailableServer> {
        let servers = self.servers.read().unwrap();
        servers.get(&id).map(|server| AvailableServer::from_server(server))
    }

    fn get_server_by_name(&self, server_name: String) -> Option<AvailableServer> {
        let servers = self.servers.read().unwrap();
        servers.iter().find(|(_, server)| server.name == server_name).map(|(_, server)| AvailableServer::from_server(server))
    }

    fn get_all_servers(&self) -> Vec<AvailableServer> {
        let servers = self.servers.read().unwrap();
        servers.iter().map(|(_, server)| AvailableServer::from_server(server)).collect()
    }

    fn reload_server(&self, db_main: &mut impl Select) {
        let mut servers = self.servers.write().unwrap();
        (*servers).init(db_main);
    }
}
