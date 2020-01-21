#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct SetItem {
  pub item_id: u32,
  pub active: bool,
  pub name: String,
}

impl PartialEq for SetItem {
  fn eq(&self, other: &Self) -> bool {
    self.item_id == other.item_id
      && self.active == other.active
      && self.name == other.name
  }
}