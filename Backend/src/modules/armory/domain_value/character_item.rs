use crate::modules::armory::dto::CharacterItemDto;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterItem {
  pub id: u32,
  pub item_id: u32,
  pub random_property_id: Option<u16>,
  pub enchant_id: Option<u32>,
  pub gem_ids: Vec<Option<u32>>
}

impl PartialEq for CharacterItem {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl CharacterItem {
  pub fn deep_eq(&self, other: &Self) -> bool {
    self.id == other.id
    && self.item_id == other.item_id
    && self.random_property_id == other.random_property_id
    && self.enchant_id == other.enchant_id
    && self.gem_ids == other.gem_ids
  }

  pub fn compare_by_value(&self, other: &CharacterItemDto) -> bool {
    return self.item_id == other.item_id
      && self.random_property_id == other.random_property_id
      && self.enchant_id == other.enchant_id
      && self.gem_ids.iter().all(|gem_flag|
          self.gem_ids.iter().filter(|x| *x == gem_flag).count()
            == other.gem_ids.iter().filter(|x| *x == gem_flag).count());
  }
}