#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Interrupt {
  pub target: u64,
  pub interrupted_spell_id: u32
}