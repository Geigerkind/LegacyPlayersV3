#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Summon {
  pub owner: u64,
  pub unit: u64
}