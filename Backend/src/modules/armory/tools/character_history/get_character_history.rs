use mysql_connection::tools::Select;

use crate::modules::armory::{
    domain_value::CharacterGuild,
    dto::{ArmoryFailure, CharacterHistoryDto},
    material::CharacterHistory,
    tools::{GetCharacter, GetCharacterFacial, GetCharacterInfo, GetGuild},
    Armory,
};

pub trait GetCharacterHistory {
    fn get_character_history(&self, character_history_id: u32) -> Result<CharacterHistory, ArmoryFailure>;
    fn get_character_history_by_value(&self, character_id: u32, character_history_dto: CharacterHistoryDto) -> Result<CharacterHistory, ArmoryFailure>;
}

impl GetCharacterHistory for Armory {
    fn get_character_history(&self, character_history_id: u32) -> Result<CharacterHistory, ArmoryFailure> {
        let character_history = self.db_main.select_wparams_value(
            "SELECT * FROM armory_character_history WHERE id=:id",
            &|mut row| {
                let character_info = self.get_character_info(row.take(2).unwrap());
                if character_info.is_err() {
                    return Err(character_info.err().unwrap());
                }
                Ok(CharacterHistory {
                    id: character_history_id,
                    character_id: row.take(1).unwrap(),
                    character_info: character_info.unwrap(),
                    character_name: row.take(3).unwrap(),
                    character_guild: row.take_opt(4).unwrap().ok().and_then(|guild_id| {
                        let guild = self.get_guild(guild_id).unwrap();
                        let rank_index: u8 = row.take(5).unwrap();
                        Some(CharacterGuild {
                            guild_id,
                            rank: guild.ranks.iter().find(|rank| rank.index == rank_index).unwrap().to_owned(),
                        })
                    }),
                    character_title: row.take_opt(6).unwrap().ok(),
                    profession_skill_points1: row.take_opt(7).unwrap().ok(),
                    profession_skill_points2: row.take_opt(8).unwrap().ok(),
                    facial: row.take_opt(9).unwrap().ok().and_then(|facial_id| self.get_character_facial(facial_id).ok()),
                    timestamp: row.take(10).unwrap(),
                })
            },
            params!(
              "id" => character_history_id
            ),
        );

        if character_history.is_none() {
            return Err(ArmoryFailure::Database("get_character_history".to_owned()));
        }
        character_history.unwrap()
    }

    fn get_character_history_by_value(&self, character_id: u32, character_history_dto: CharacterHistoryDto) -> Result<CharacterHistory, ArmoryFailure> {
        let guild_id = if character_history_dto.character_guild.is_some() {
            let character = self.get_character(character_id).unwrap();
            let guild = self.get_guild_by_uid(character.server_id, character_history_dto.character_guild.as_ref().unwrap().guild.server_uid);
            if guild.is_none() {
                return Err(ArmoryFailure::InvalidInput);
            }
            Some(guild.unwrap().id)
        } else {
            None
        };

        let character_info_res = self.get_character_info_by_value(character_history_dto.character_info.to_owned());
        if character_info_res.is_err() {
            return Err(character_info_res.err().unwrap());
        }

        let facial = character_history_dto.facial.as_ref().and_then(|facial_dto| self.get_character_facial_by_value(facial_dto.clone()).ok());

        let params = params!(
          "character_id" => character_id,
          "character_info_id" => character_info_res.as_ref().unwrap().id,
          "character_name" => character_history_dto.character_name.clone(),
          "title" => character_history_dto.character_title,
          "guild_id" => guild_id,
          "guild_rank" => character_history_dto.character_guild.as_ref().map(|chr_guild_dto| chr_guild_dto.rank.index),
          "prof_skill_points1" => character_history_dto.profession_skill_points1,
          "prof_skill_points2" => character_history_dto.profession_skill_points2,
          "facial" => facial.as_ref().map(|chr_facial| chr_facial.id)
        );

        self.db_main
            .select_wparams_value(
                "SELECT id, timestamp FROM armory_character_history WHERE character_id=:character_id AND character_info_id=:character_info_id AND character_name=:character_name AND ((ISNULL(:guild_id) AND ISNULL(guild_id)) OR guild_id = :guild_id) \
                 AND ((ISNULL(:guild_rank) AND ISNULL(guild_rank)) OR guild_rank = :guild_rank) AND ((ISNULL(:title) AND ISNULL(title)) OR title = :title) AND ((ISNULL(:prof_skill_points1) AND ISNULL(prof_skill_points1)) OR prof_skill_points1 = \
                 :prof_skill_points1) AND ((ISNULL(:prof_skill_points2) AND ISNULL(prof_skill_points2)) OR prof_skill_points2 = :prof_skill_points2) AND ((ISNULL(:facial) AND ISNULL(facial)) OR facial = :facial) AND timestamp >= UNIX_TIMESTAMP()-60",
                &|mut row| {
                    Ok(CharacterHistory {
                        id: row.take(0).unwrap(),
                        character_id,
                        character_info: character_info_res.as_ref().unwrap().to_owned(),
                        character_name: character_history_dto.character_name.to_owned(),
                        character_guild: guild_id.map(|id| CharacterGuild {
                            guild_id: id,
                            rank: character_history_dto.character_guild.as_ref().map(|chr_guild_dto| chr_guild_dto.rank.clone()).unwrap(),
                        }),
                        character_title: character_history_dto.character_title,
                        profession_skill_points1: character_history_dto.profession_skill_points1,
                        profession_skill_points2: character_history_dto.profession_skill_points2,
                        facial: facial.to_owned(),
                        timestamp: row.take(1).unwrap(),
                    })
                },
                params,
            )
            .unwrap_or_else(|| Err(ArmoryFailure::Database("get_character_history_by_value".to_owned())))
    }
}
