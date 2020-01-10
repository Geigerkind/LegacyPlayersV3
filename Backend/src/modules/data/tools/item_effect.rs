use crate::modules::data::Data;
use crate::modules::data::domain_value::ItemEffect;

pub trait RetrieveItemEffect {
  fn get_item_effect(&self, expansion_id: u8, item_id: u32) -> Option<ItemEffect>;
}

impl RetrieveItemEffect for Data {
  fn get_item_effect(&self, expansion_id: u8, item_id: u32) -> Option<ItemEffect> {
    if expansion_id == 0 {
      return None;
    }

    self.item_effects.get(expansion_id as usize - 1)
      .and_then(|map| map.get(&item_id)
        .and_then(|item_effect| Some(item_effect.clone())))
  }
}