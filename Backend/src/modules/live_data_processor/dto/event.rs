#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Event {
  pub unit: u64,
  pub event_type: u8
}