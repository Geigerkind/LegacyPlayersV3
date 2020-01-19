#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct SetItem {
  pub item_id: u32,
  pub active: bool,
  pub name: String,
}