use crate::modules::live_data_processor::domain_value::Unit;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuraApplication {
  pub caster: Unit,
  pub stack_amount: u32,
  pub spell_id: u32
}