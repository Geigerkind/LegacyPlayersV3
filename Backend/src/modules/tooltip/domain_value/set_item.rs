#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct SetItem {
  pub active: bool,
  pub name: String,
}