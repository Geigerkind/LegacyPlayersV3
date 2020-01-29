#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterItemDto {
  pub item_id: u32,
  pub random_property_id: Option<i16>,
  pub random_property_scaling_factor: Option<u16>,
  pub enchant_id: Option<u32>,
  pub gem_ids: Vec<Option<u32>>
}