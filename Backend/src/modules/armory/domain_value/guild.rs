#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Guild {
  pub id: u32,
  pub server_id: u32,
  pub name: String
}