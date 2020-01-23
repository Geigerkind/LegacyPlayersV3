use mysql_connection::tools::{Execute, Select};

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::dto::CharacterHistoryDto;
use crate::modules::armory::material::CharacterHistory;
use crate::modules::armory::tools::{CreateCharacterInfo, CreateGuild, GetCharacter, CreateCharacterFacial};
use crate::modules::armory::domain_value::CharacterGuild;

pub trait CreateCharacterHistory {
  fn create_character_history(&self, server_id: u32, character_history_dto: CharacterHistoryDto, character_uid: u64) -> Result<CharacterHistory, Failure>;
}

impl CreateCharacterHistory for Armory {
  // Assumption: It has been checked that the previous value is not the same
  // Assumption: Character exists
  fn create_character_history(&self, server_id: u32, character_history_dto: CharacterHistoryDto, character_uid: u64) -> Result<CharacterHistory, Failure> {
    let character_id = self.get_character_id_by_uid(server_id, character_uid).unwrap();
    let mut guild_id = None;
    if character_history_dto.character_guild.is_some() {
      let guild = self.create_guild(server_id, character_history_dto.character_guild.as_ref().unwrap().guild.to_owned());
      if guild.is_err() {
        return Err(guild.err().unwrap());
      }
      guild_id = Some(guild.unwrap().id);
    }
    let character_info_res = self.create_character_info(character_history_dto.character_info.to_owned());
    if character_info_res.is_err() {
      return Err(character_info_res.err().unwrap());
    }
    let character_info = character_info_res.unwrap();

    let mut facial = None;
    if character_history_dto.facial.is_some() {
      let facial_res = self.create_character_facial(character_history_dto.facial.as_ref().unwrap().clone());
      if facial_res.is_err() {
        return Err(facial_res.err().unwrap());
      }
      facial = facial_res.ok();
    }

    let mut characters = self.characters.write().unwrap();
    let params = params!(
      "character_id" => character_id,
      "character_info_id" => character_info.id,
      "character_name" => character_history_dto.character_name.clone(),
      "title" => character_history_dto.character_title.clone(),
      "guild_id" => guild_id.clone(),
      "guild_rank" => character_history_dto.character_guild.as_ref().and_then(|chr_guild_dto| Some(chr_guild_dto.rank.clone())),
      "prof_skill_points1" => character_history_dto.profession_skill_points1.clone(),
      "prof_skill_points2" => character_history_dto.profession_skill_points2.clone(),
      "facial" => facial.as_ref().and_then(|chr_facial| Some(chr_facial.id.clone()))
    );

    if self.db_main.execute_wparams("INSERT INTO armory_character_history (`character_id`, `character_info_id`, `character_name`, `title`, `guild_id`, `guild_rank`, `prof_skill_points1`, `prof_skill_points2`, `facial`, `timestamp`) VALUES (:character_id, :character_info_id, :character_name, :title, :guild_id, :guild_rank, :prof_skill_points1, :prof_skill_points2, :facial, UNIX_TIMESTAMP())", params.clone()) {
      let character_history_res = self.db_main.select_wparams_value("SELECT id, timestamp FROM armory_character_history WHERE character_id=:character_id AND character_info_id=:character_info_id AND character_name=:character_name \
      AND ((ISNULL(:guild_id) AND ISNULL(guild_id)) OR guild_id = :guild_id) \
      AND ((ISNULL(:guild_rank) AND ISNULL(guild_rank)) OR guild_rank = :guild_rank) \
      AND ((ISNULL(:title) AND ISNULL(title)) OR title = :title) \
      AND ((ISNULL(:prof_skill_points1) AND ISNULL(prof_skill_points1)) OR prof_skill_points1 = :prof_skill_points1) \
      AND ((ISNULL(:prof_skill_points2) AND ISNULL(prof_skill_points2)) OR prof_skill_points2 = :prof_skill_points2) \
      AND ((ISNULL(:facial) AND ISNULL(facial)) OR facial = :facial) \
      AND timestamp >= UNIX_TIMESTAMP()-60", &|mut row| {
        CharacterHistory {
          id: row.take(0).unwrap(),
          character_id,
          character_info: character_info.to_owned(),
          character_name: character_history_dto.character_name.to_owned(),
          character_guild: guild_id.and_then(|id| Some(CharacterGuild {
            guild_id: id.clone(),
            rank: character_history_dto.character_guild.as_ref().and_then(|chr_guild_dto| Some(chr_guild_dto.rank.clone())).unwrap()
          })),
          character_title: character_history_dto.character_title,
          profession_skill_points1: character_history_dto.profession_skill_points1,
          profession_skill_points2: character_history_dto.profession_skill_points2,
          facial: facial.to_owned(),
          timestamp: row.take(1).unwrap(),
        }
      }, params);
      characters.get_mut(&character_id).unwrap().last_update = character_history_res.clone();
      if character_history_res.is_some() {
        return Ok(character_history_res.unwrap());
      }
    }

    Err(Failure::Unknown)
  }
}