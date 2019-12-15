use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct ProlongToken {
  pub token_id: u32,
  pub days: u32,
}