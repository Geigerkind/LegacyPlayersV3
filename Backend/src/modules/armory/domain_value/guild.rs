#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Guild {
  pub id: u32,
  pub server_id: u32,
  pub name: String
}