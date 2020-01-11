use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::dto::CharacterHistoryDto;
use crate::modules::armory::material::CharacterHistory;

pub trait CreateCharacterHistory {
  fn create_character_history(&self, character_history_dto: CharacterHistoryDto) -> Result<CharacterHistory, Failure>;
}

impl CreateCharacterHistory for Armory {
  fn create_character_history(&self, character_history_dto: CharacterHistoryDto) -> Result<CharacterHistory, Failure> {
    unimplemented!()
  }
}