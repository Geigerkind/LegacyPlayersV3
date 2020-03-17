use crate::modules::data::{dto::AvailableServer, Data};

pub trait RetrieveServer {
    fn get_server(&self, id: u32) -> Option<AvailableServer>;
    fn get_server_by_name(&self, server_name: String) -> Option<AvailableServer>;
    fn get_all_servers(&self) -> Vec<AvailableServer>;
}

impl RetrieveServer for Data {
    fn get_server(&self, id: u32) -> Option<AvailableServer> {
        self.servers.get(&id).map(|server| AvailableServer::from_server(server))
    }

    fn get_server_by_name(&self, server_name: String) -> Option<AvailableServer> {
        self.servers.iter().find(|(_, server)| server.name == server_name).map(|(_, server)| AvailableServer::from_server(server))
    }

    fn get_all_servers(&self) -> Vec<AvailableServer> {
        self.servers.iter().map(|(_, server)| AvailableServer::from_server(server)).collect()
    }
}
