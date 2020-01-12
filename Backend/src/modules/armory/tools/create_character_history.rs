use mysql_connection::tools::{Execute, Select};

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::dto::CharacterHistoryDto;
use crate::modules::armory::material::CharacterHistory;
use crate::modules::armory::tools::{CreateCharacterInfo, GetCharacter, GetGuild};

pub trait CreateCharacterHistory {
  fn create_character_history(&self, server_id: u32, character_history_dto: CharacterHistoryDto) -> Result<CharacterHistory, Failure>;
}

impl CreateCharacterHistory for Armory {
  // Assumptions:
  // Char exists
  // It has been checked that the previous value is not the same
  fn create_character_history(&self, server_id: u32, character_history_dto: CharacterHistoryDto) -> Result<CharacterHistory, Failure> {
    let character_id = self.get_character_id_by_uid(server_id, character_history_dto.character_uid).unwrap();
    let guild_id = character_history_dto.guild_name.as_ref().and_then(|guild_name| self.get_guild_id_by_name(server_id, guild_name.clone()));
    let character_info_res = self.create_character_info(character_history_dto.character_info.to_owned());
    if character_info_res.is_err() {
      return Err(character_info_res.err().unwrap());
    }
    let character_info = character_info_res.unwrap();

    let mut characters = self.characters.write().unwrap();
    let params = params!(
      "character_id" => character_id,
      "character_info_id" => character_info.id,
      "character_name" => character_history_dto.character_name.clone(),
      "guild_id" => guild_id.clone(),
      "guild_rank" => character_history_dto.guild_rank.clone()
    );
    if self.db_main.execute_wparams("INSERT INTO armory_character_history (`character_id`, `character_info_id`, `character_name`, `guild_id`, `guild_rank`, `timestamp`) VALUES (:character_id, :character_info_id, :character_name, :guild_id, :guild_rank, UNIX_TIMESTAMP())", params.clone()) {
      let character_history_res = self.db_main.select_wparams_value("SELECT id, timestamp FROM armory_character_history WHERE character_id=:character_id AND character_info_id=:character_info_id AND character_name=:character_name AND guild_id=:guild_id AND guild_rank=:guild_rank AND timestamp >= UNIX_TIMESTAMP()-60", &|mut row| {
        CharacterHistory {
          id: row.take(0).unwrap(),
          character_id,
          character_info: character_info.to_owned(),
          character_name: character_history_dto.character_name.to_owned(),
          guild_id,
          guild_rank: character_history_dto.guild_rank.to_owned(),
          timestamp: row.take(1).unwrap(),
        }
      }, params);
      characters.get_mut(&character_id).unwrap().last_update = character_history_res;
    }

    Err(Failure::Unknown)
  }
}