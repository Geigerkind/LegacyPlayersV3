use crate::modules::live_data_processor::domain_value::Mitigation;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Heal {
  pub total: u32,
  pub effective: u32,
  pub mitigation: Vec<Mitigation>
}