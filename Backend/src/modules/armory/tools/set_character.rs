use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::dto::{CharacterDto, CharacterHistoryDto};
use crate::modules::armory::material::{Character, CharacterHistory};
use crate::modules::armory::tools::{CreateCharacter, GetCharacter, GetGuild, CreateCharacterHistory};

pub trait SetCharacter {
  fn set_character(&self, server_id: u32, update_character: CharacterDto) -> Result<Character, Failure>;
  fn set_character_history(&self, server_id: u32, update_character_history: CharacterHistoryDto) -> Result<CharacterHistory, Failure>;
}

impl SetCharacter for Armory {
  fn set_character(&self, server_id: u32, update_character: CharacterDto) -> Result<Character, Failure> {
    // Validation
    if update_character.server_uid == 0 {
      return Err(Failure::InvalidInput);
    }

    // Exists the character ? get char : create char
    let character_id_res = self.get_character_id_by_uid(server_id, update_character.server_uid)
      .or_else(|| self.create_character(server_id, update_character.server_uid).ok());
    if character_id_res.is_none() {
      return Err(Failure::Unknown);
    }
    let character_id = character_id_res.unwrap();

    // Set the character history
    if update_character.character_history.is_some() {
      let character_history_res = self.set_character_history(server_id, update_character.character_history.unwrap());
      if character_history_res.is_err() {
        return Err(character_history_res.err().unwrap());
      }
    }

    self.get_character(character_id).ok_or(Failure::Unknown)
  }

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

    // Check whether this is a new entry or just the same as previously
    let mut characters = self.characters.write().unwrap();
    let character = characters.get_mut(&character_id).unwrap();

    if character.last_update.is_some() {
      let mut last_update = character.last_update.as_mut().unwrap();
      if last_update.guild_id == guild_id
        && last_update.guild_rank == update_character_history.guild_rank
        && last_update.character_name == update_character_history.character_name
        && last_update.character_info.compare_by_value(&update_character_history.character_info)
      {
        last_update.timestamp = time_util::now();
        return Ok(last_update.clone());
      }
    } // Else create a new history point and assign it to this character
    self.create_character_history(update_character_history)
  }
}