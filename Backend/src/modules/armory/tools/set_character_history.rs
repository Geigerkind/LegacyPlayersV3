use mysql_connection::tools::Execute;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::dto::CharacterHistoryDto;
use crate::modules::armory::material::CharacterHistory;
use crate::modules::armory::tools::{CreateCharacterHistory, GetCharacter, CreateGuild};

pub trait SetCharacterHistory {
  fn set_character_history(&self, server_id: u32, update_character_history: CharacterHistoryDto) -> Result<CharacterHistory, Failure>;
}

impl SetCharacterHistory for Armory {
  fn set_character_history(&self, server_id: u32, update_character_history: CharacterHistoryDto) -> Result<CharacterHistory, Failure> {
    // Validation
    if update_character_history.character_name.is_empty()
      || update_character_history.guild_rank.contains(&String::new()) {
      return Err(Failure::InvalidInput);
    } else if update_character_history.guild.is_some() {
      let guild = update_character_history.guild.as_ref().unwrap();
      if guild.server_uid == 0 || guild.name.is_empty() {
        return Err(Failure::InvalidInput);
      }
    }

    // Check if this character exists
    let character_id_res = self.get_character_id_by_uid(server_id, update_character_history.character_uid);
    if character_id_res.is_none() {
      return Err(Failure::InvalidInput);
    }

    let character_id = character_id_res.unwrap();
    let guild_id = update_character_history.guild.as_ref().and_then(|guild_dto| self.create_guild(server_id, guild_dto.clone()).ok().and_then(|gld| Some(gld.id)));

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
          let now = time_util::now();
          if self.db_main.execute_wparams("UPDATE armory_character_history SET `timestamp` = :timestamp WHERE id=:id", params!(
            "timestamp" => now.clone(),
            "id" => last_update.id
          )) {
            last_update.timestamp = now.to_owned();
            return Ok(last_update.clone());
          }
          return Err(Failure::Unknown);
        }
      }
    } // Else create a new history point and assign it to this character
    self.create_character_history(server_id, update_character_history)
  }
}