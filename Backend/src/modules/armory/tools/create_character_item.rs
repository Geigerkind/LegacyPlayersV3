use crate::dto::Failure;
use crate::modules::armory::domain_value::CharacterItem;
use crate::modules::armory::Armory;

pub trait CreateCharacterItem {
  // Note: CharacterItem = CharacterItemDto here
  fn create_character_item(&self, character_item: CharacterItem) -> Result<CharacterItem, Failure>;
}

impl CreateCharacterItem for Armory {
  fn create_character_item(&self, character_item: CharacterItem) -> Result<CharacterItem, Failure> {
    unimplemented!()
  }
}