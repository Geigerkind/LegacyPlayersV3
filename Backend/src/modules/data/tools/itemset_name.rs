use crate::modules::data::Data;
use crate::modules::data::domain_value::ItemsetName;

pub trait RetrieveItemsetName {
  fn get_itemset_name(&self, expansion_id: u8, itemset_id: u16) -> Option<ItemsetName>;
}

impl RetrieveItemsetName for Data {
  fn get_itemset_name(&self, expansion_id: u8, itemset_id: u16) -> Option<ItemsetName> {
    if expansion_id == 0 {
      return None;
    }

    self.itemset_names.get(expansion_id as usize - 1)
      .and_then(|map| map.get(&itemset_id)
        .and_then(|itemset_name| Some(itemset_name.clone())))
  }
}