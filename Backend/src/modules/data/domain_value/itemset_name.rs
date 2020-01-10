#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ItemsetName {
  pub expansion_id: u8,
  pub id: u16,
  pub localization_id: u32
}