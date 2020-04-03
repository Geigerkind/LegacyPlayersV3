#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpellCast {
  pub caster: u64,
  pub target: Option<u64>,
  pub spell_id: u32,
  pub hit_type: u8
}