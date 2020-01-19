use crate::modules::tooltip::domain_value::{Stat, ItemSet, Socket};

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ItemTooltip {
  pub name: String,
  pub icon: String,
  pub quality: u8,
  pub bonding: String,
  pub inventory_type: String,
  pub sheath_type: String,
  pub sub_class: String,
  pub armor: Option<u16>,
  pub stats: Vec<Stat>,
  pub durability: Option<u16>,
  pub item_level: Option<u16>,
  pub required_level: Option<u16>,
  pub item_effects: Vec<String>,
  pub item_set: Option<ItemSet>,
  pub socket: Option<Socket>,
  pub enchant: Option<String>
}