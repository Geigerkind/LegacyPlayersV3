use mysql_connection::tools::Execute;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::CharacterItem;
use crate::modules::armory::dto::CharacterItemDto;
use crate::modules::armory::tools::GetCharacterItem;

pub trait CreateCharacterItem {
  fn create_character_item(&self, character_item: CharacterItemDto) -> Result<CharacterItem, Failure>;
}

impl CreateCharacterItem for Armory {
  fn create_character_item(&self, character_item: CharacterItemDto) -> Result<CharacterItem, Failure> {
    // If it already exists, return this one
    let existing_item = self.get_character_item_by_value(character_item.clone());
    if existing_item.is_ok() {
      return existing_item;
    }

    let params = params!(
      "item_id" => character_item.item_id,
      "random_property_id" => character_item.random_property_id,
      "enchant_id" => character_item.enchant_id,
      "gem_id1" => character_item.gem_ids.get(0).cloned().unwrap(),
      "gem_id2" => character_item.gem_ids.get(1).cloned().unwrap(),
      "gem_id3" => character_item.gem_ids.get(2).cloned().unwrap(),
      "gem_id4" => character_item.gem_ids.get(3).cloned().unwrap()
    );
    if self.db_main.execute_wparams("INSERT INTO armory_item (`item_id`, `random_property_id`, `enchant_id`, `gem_id1`, `gem_id2`, `gem_id3`, `gem_id4`) VALUES (:item_id, :random_property_id, :enchant_id, :gem_id1, :gem_id2, :gem_id3, :gem_id4)", params.clone()) {
      return self.get_character_item_by_value(character_item);
    }

    Err(Failure::Unknown)
  }
}