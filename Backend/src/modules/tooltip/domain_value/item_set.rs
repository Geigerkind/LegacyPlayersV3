use crate::modules::tooltip::domain_value::{SetItem, SetEffect};

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ItemSet {
  pub name: String,
  pub set_items: Vec<SetItem>,
  pub set_effects: Vec<SetEffect>
}