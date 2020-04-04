#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Power {
  pub unit: u64,
  pub power_type: u8,
  pub max_power: u32,
  pub current_power: u32
}