use crate::modules::data::Data;
use crate::modules::data::domain_value::ItemRandomProperty;

pub trait RetrieveItemRandomProperty {
  fn get_item_random_property(&self, expansion_id: u8, random_property_id: u32) -> Option<ItemRandomProperty>;
}

impl RetrieveItemRandomProperty for Data {
  fn get_item_random_property(&self, expansion_id: u8, random_property_id: u32) -> Option<ItemRandomProperty> {
    if expansion_id == 0 {
      return None;
    }

    self.item_random_properties.get(expansion_id as usize - 1)
      .and_then(|map| map.get(&random_property_id)
        .and_then(|item_random_property| Some(item_random_property.clone())))
  }
}