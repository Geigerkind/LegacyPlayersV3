use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::dto::CharacterHistoryDto;
use crate::modules::armory::material::CharacterHistory;
use crate::modules::armory::tools::{CreateCharacterHistory, GetCharacter, GetGuild};

pub trait SetCharacterHistory {
  fn set_character_history(&self, server_id: u32, update_character_history: CharacterHistoryDto) -> Result<CharacterHistory, Failure>;
}

impl SetCharacterHistory for Armory {
  fn set_character_history(&self, server_id: u32, update_character_history: CharacterHistoryDto) -> Result<CharacterHistory, Failure> {
    // Validation
    if update_character_history.character_name.is_empty()
      || update_character_history.guild_name.contains(&String::new())
      || update_character_history.guild_rank.contains(&String::new()) {
      return Err(Failure::InvalidInput);
    }

    // Check if this character exists
    let character_id_res = self.get_character_id_by_uid(server_id, update_character_history.character_uid);
    if character_id_res.is_none() {
      return Err(Failure::InvalidInput);
    }

    let character_id = character_id_res.unwrap();
    let guild_id = update_character_history.guild_name.as_ref().and_then(|guild_name| self.get_guild_id_by_name(server_id, guild_name.clone()));

    { // Check whether this is a new entry or just the same as previously
      let mut characters = self.characters.write().unwrap();
      let character = characters.get_mut(&character_id).unwrap();

      if character.last_update.is_some() {
        let mut last_update = character.last_update.as_mut().unwrap();
        if last_update.guild_id == guild_id
          && last_update.guild_rank == update_character_history.guild_rank
          && last_update.character_name == update_character_history.character_name
          && last_update.character_info.compare_by_value(&update_character_history.character_info)
        {
          // TODO: Update DB as well
          last_update.timestamp = time_util::now();
          return Ok(last_update.clone());
        }
      }
    } // Else create a new history point and assign it to this character
    self.create_character_history(server_id, update_character_history)
  }
}