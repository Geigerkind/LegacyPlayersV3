use crate::modules::live_data_processor::dto::Unit;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Position {
  pub map_id: u32,
  pub instance_id: u32,
  pub map_difficulty: u8,
  pub unit: Unit,
  pub x: i32,
  pub y: i32,
  pub z: i32,
  pub orientation: i32
}