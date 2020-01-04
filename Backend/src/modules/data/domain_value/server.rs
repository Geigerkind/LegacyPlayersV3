#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Server {
  pub id: u32,
  pub expansion_id: u8,
  pub name: String
}