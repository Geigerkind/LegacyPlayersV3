use crate::modules::data::domain_value::Server;

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AvailableServer {
  pub id: u32,
  pub expansion_id: u8,
  pub name: String
}

impl AvailableServer {
  pub fn from_server(server: &Server) -> AvailableServer {
    AvailableServer {
      id: server.id,
      expansion_id: server.expansion_id,
      name: server.name.clone()
    }
  }
}