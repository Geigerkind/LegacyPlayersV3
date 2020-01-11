use crate::dto::Failure;
use crate::modules::armory::domain_value::CharacterInfo;
use crate::modules::armory::Armory;

pub trait CreateCharacterInfo {
  // Here CharacterInfo = CharacterInfoDto
  fn create_character_info(&self, character_info: CharacterInfo) -> Result<CharacterInfo, Failure>;
}

impl CreateCharacterInfo for Armory {
  fn create_character_info(&self, character_info: CharacterInfo) -> Result<CharacterInfo, Failure> {
    unimplemented!()
  }
}