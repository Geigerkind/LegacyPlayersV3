use crate::modules::live_data_processor::domain_value::Unit;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InterruptedSpell {
  pub target: Option<Unit>,
  pub spell_id: u32
}