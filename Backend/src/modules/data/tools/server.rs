use crate::modules::data::Data;
use crate::modules::data::domain_value::Server;

pub trait RetrieveServer {
  fn get_server(&self, id: u32) -> Option<Server>;
  fn get_all_servers(&self) -> Vec<Server>;
}

impl RetrieveServer for Data {
  fn get_server(&self, id: u32) -> Option<Server> {
    self.servers.get(&id)
      .and_then(|server| Some(server.clone()))
  }

  fn get_all_servers(&self) -> Vec<Server> {
    self.servers.iter().map(|(_, server)| server.clone()).collect()
  }
}
