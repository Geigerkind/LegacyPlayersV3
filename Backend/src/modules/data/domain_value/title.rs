#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Title {
  pub id: u16,
  pub localization_id: u32
}