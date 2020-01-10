#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ItemRandomProperty {
  pub expansion_id: u8,
  pub id: u32,
  pub localization_id: u32,
  pub enchant_ids: Vec<u32>
}