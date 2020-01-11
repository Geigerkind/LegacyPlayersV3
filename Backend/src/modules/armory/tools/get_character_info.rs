use crate::modules::armory::domain_value::CharacterInfo;
use crate::dto::Failure;
use crate::modules::armory::Armory;

pub trait GetCharacterInfo {
  fn get_character_info(&self, character_info_id: u32) -> Result<CharacterInfo, Failure>;
  fn get_character_info_by_value(&self, character_info_values: CharacterInfo) -> Result<CharacterInfo, Failure>;
}

impl GetCharacterInfo for Armory {
  fn get_character_info(&self, character_info_id: u32) -> Result<CharacterInfo, Failure> {
    unimplemented!()
  }

  fn get_character_info_by_value(&self, character_info_values: CharacterInfo) -> Result<CharacterInfo, Failure> {
    unimplemented!()
  }
}