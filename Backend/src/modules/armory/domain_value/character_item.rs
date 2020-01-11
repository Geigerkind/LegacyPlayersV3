#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterItem {
  pub id: u32,
  pub item_id: u32,
  pub random_property_id: Option<u16>,
  pub enchant_id: Option<u32>,
  pub gem_ids: Vec<Option<u32>>
}

impl CharacterItem {
  pub fn compare_by_value(&self, other: &CharacterItem) -> bool {
    return self.item_id == other.item_id
      && self.random_property_id == other.random_property_id
      && self.enchant_id == other.enchant_id
      && self.gem_ids.iter().all(|gem_flag|
          self.gem_ids.iter().filter(|x| *x == gem_flag).count()
            == other.gem_ids.iter().filter(|x| *x == gem_flag).count());
  }
}