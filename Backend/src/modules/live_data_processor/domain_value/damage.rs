use crate::modules::live_data_processor::domain_value::{School, Mitigation};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Damage {
  school: School,
  damage: u32,
  mitigation: Vec<Mitigation>
}