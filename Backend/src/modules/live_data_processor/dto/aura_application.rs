#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct AuraApplication {
  pub caster: u64,
  pub target: u64,
  pub spell_id: u32,
  pub stack_amount: u8,
  pub applied: bool
}