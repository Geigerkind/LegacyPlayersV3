#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Loot {
  pub unit: u64,
  pub item_id: u32
}