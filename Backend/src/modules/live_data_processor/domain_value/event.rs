use crate::modules::live_data_processor::domain_value::{Unit, EventType};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Event {
  pub id: u32,
  pub timestamp: u64,
  pub subject: Unit,
  pub event: EventType
}