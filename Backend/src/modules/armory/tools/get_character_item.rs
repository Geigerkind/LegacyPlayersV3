use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::CharacterItem;
use mysql_connection::tools::Select;

pub trait GetCharacterItem {
  fn get_character_item(&self, character_item_id: u32) -> Result<CharacterItem, Failure>;
  // Here CharacterItem = CharacterItemDto
  fn get_character_item_by_value(&self, character_item: CharacterItem) -> Result<CharacterItem, Failure>;
}

impl GetCharacterItem for Armory {
  fn get_character_item(&self, character_item_id: u32) -> Result<CharacterItem, Failure> {
    let params = params!(
      "id" => character_item_id
    );
    self.db_main.select_wparams_value("SELECT * FROM armory_item WHERE id=:id", &|mut row| {
      CharacterItem {
        id: row.take(0).unwrap(),
        item_id: row.take(1).unwrap(),
        random_property_id: row.take_opt(2).unwrap().ok(),
        enchant_id: row.take_opt(3).unwrap().ok(),
        gem_ids: vec![
          row.take_opt(4).unwrap().ok(),
          row.take_opt(5).unwrap().ok(),
          row.take_opt(6).unwrap().ok(),
          row.take_opt(7).unwrap().ok()
        ]
      }
    }, params);
    Err(Failure::Unknown)
  }

  fn get_character_item_by_value(&self, character_item: CharacterItem) -> Result<CharacterItem, Failure> {
    let params = params!(
      "item_id" => character_item.item_id,
      "random_property_id" => character_item.random_property_id,
      "enchant_id" => character_item.enchant_id,
      "gem_id1" => character_item.gem_ids.get(0).cloned().unwrap(),
      "gem_id2" => character_item.gem_ids.get(1).cloned().unwrap(),
      "gem_id3" => character_item.gem_ids.get(2).cloned().unwrap(),
      "gem_id4" => character_item.gem_ids.get(3).cloned().unwrap(),
    );
    self.db_main.select_wparams_value("SELECT id FROM armory_item WHERE item_id=:item_id AND random_property_id=:random_property_id AND enchant_id=:enchant_id AND gem_id1=:gem_id1 AND gem_id2=:gem_id2 AND gem_id3=:gem_id3 AND gem_id4=:gem_id4", &|mut row| {
      let mut new_character_item = character_item.to_owned();
      new_character_item.id = row.take(0).unwrap();
      new_character_item
    }, params);
    Err(Failure::Unknown)
  }
}