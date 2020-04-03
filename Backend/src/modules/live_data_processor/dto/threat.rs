#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Threat {
  pub threater: u64,
  pub threatened: u64,
  pub spell_id: u32,
  pub amount: i32
}