#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Guild {
  pub id: u32,
  pub server_id: u32,
  pub server_uid: u64,
  pub name: String
}