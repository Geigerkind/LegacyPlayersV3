#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Instance {
  pub map_id: u32,
  pub instance_id: u32,
  pub winner: Option<u8>
}