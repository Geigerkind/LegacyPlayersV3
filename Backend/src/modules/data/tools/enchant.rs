use crate::modules::data::Data;
use crate::modules::data::material::Enchant;

pub trait RetrieveEnchant {
  fn get_enchant(&self, expansion_id: u8, enchant_id: u32) -> Option<Enchant>;
}

impl RetrieveEnchant for Data {
  fn get_enchant(&self, expansion_id: u8, enchant_id: u32) -> Option<Enchant> {
    if expansion_id == 0 {
      return None;
    }

    self.enchants.get(expansion_id as usize - 1).and_then(|map| map.get(&enchant_id).and_then(|enchant| Some(enchant.clone())))
  }
}