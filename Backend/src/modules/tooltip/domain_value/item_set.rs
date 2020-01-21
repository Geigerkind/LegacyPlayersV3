use crate::modules::tooltip::domain_value::SetEffect;
use crate::modules::tooltip::material::SetItem;

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ItemSet {
  pub name: String,
  pub set_items: Vec<SetItem>,
  pub set_effects: Vec<SetEffect>
}