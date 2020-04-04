#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Interrupt {
  pub target: u64,
  pub interrupted_spell_id: u32
}