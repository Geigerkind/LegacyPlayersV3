#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Death {
  pub cause: u64,
  pub victim: u64
}