#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct CharacterItem {
  pub id: u32,
  pub item_id: u32,
  pub random_property_id: Option<u16>,
  pub enchant_id: Option<u32>,
  pub gem_ids: Vec<Option<u32>>
}