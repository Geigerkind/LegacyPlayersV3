#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Death {
  pub cause: u64,
  pub victim: u64
}