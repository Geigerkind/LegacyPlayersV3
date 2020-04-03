#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HealDone {
  pub caster: u64,
  pub target: u64,
  pub spell_id: u32,
  pub total_heal: u32,
  pub effective_heal: u32,
  pub absorb: u32
}