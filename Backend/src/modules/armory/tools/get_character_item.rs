use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::CharacterItem;

pub trait GetCharacterItem {
  fn get_character_item(&self, character_item_id: u32) -> Result<CharacterItem, Failure>;
}

impl GetCharacterItem for Armory {
  fn get_character_item(&self, character_item_id: u32) -> Result<CharacterItem, Failure> {
    unimplemented!()
  }
}