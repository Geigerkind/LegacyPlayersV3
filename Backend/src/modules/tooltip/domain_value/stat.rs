#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Stat {
  pub value: u16,
  pub name: String
}