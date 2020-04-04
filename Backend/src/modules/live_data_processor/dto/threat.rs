#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Threat {
  pub threater: u64,
  pub threatened: u64,
  pub spell_id: Option<u32>,
  pub amount: i32
}