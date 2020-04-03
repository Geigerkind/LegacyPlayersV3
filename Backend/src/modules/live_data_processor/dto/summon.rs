#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Summon {
  pub owner: u64,
  pub unit: u64
}