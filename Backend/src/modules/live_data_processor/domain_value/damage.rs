use crate::modules::live_data_processor::domain_value::{School, Mitigation};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Damage {
  pub school: School,
  pub damage: u32,
  pub mitigation: Vec<Mitigation>
}