#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Creature {
  pub creature_id: u32,
  pub entry: u32,
  pub owner: Option<u32>
}