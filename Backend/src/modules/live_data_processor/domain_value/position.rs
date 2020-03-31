#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Position {
  pub x: u32,
  pub y: u32,
  pub z: u32,
  pub orientation: u32
}