use mysql_connection::tools::Execute;

use crate::dto::CheckPlausability;
use crate::modules::armory::dto::{ArmoryFailure, CharacterHistoryDto};
use crate::modules::armory::material::CharacterHistory;
use crate::modules::armory::tools::{CreateCharacterHistory, CreateGuild, GetCharacter};
use crate::modules::armory::Armory;

pub trait SetCharacterHistory {
    fn set_character_history(
        &self,
        server_id: u32,
        update_character_history: CharacterHistoryDto,
        uid: u64,
    ) -> Result<CharacterHistory, ArmoryFailure>;
}

impl SetCharacterHistory for Armory {
    fn set_character_history(
        &self,
        server_id: u32,
        update_character_history: CharacterHistoryDto,
        character_uid: u64,
    ) -> Result<CharacterHistory, ArmoryFailure> {
        // Validation
        if !update_character_history.is_plausible() {
            return Err(ArmoryFailure::ImplausibleInput);
        }

        // Check if this character exists
        let character_id_res = self.get_character_id_by_uid(server_id, character_uid);
        if character_id_res.is_none() {
            return Err(ArmoryFailure::InvalidInput);
        }

        let character_id = character_id_res.unwrap();
        let guild_id =
            update_character_history
                .character_guild
                .as_ref()
                .and_then(|chr_guild_dto| {
                    self.create_guild(server_id, chr_guild_dto.guild.clone())
                        .ok()
                        .map(|gld| gld.id)
                });

        {
            // Check whether this is a new entry or just the same as previously
            let mut characters = self.characters.write().unwrap();
            let character = characters.get_mut(&character_id).unwrap();

            if character.last_update.is_some() {
                let mut last_update = character.last_update.as_mut().unwrap();
                if last_update.compare_by_value(&update_character_history)
                    && ((last_update.character_guild.is_none() && guild_id.is_none())
                        || (last_update.character_guild.is_some()
                            && guild_id.is_some()
                            && last_update.character_guild.as_ref().unwrap().guild_id
                                == *guild_id.as_ref().unwrap()))
                {
                    let now = time_util::now();
                    if self.db_main.execute_wparams(
                        "UPDATE armory_character_history SET `timestamp` = :timestamp WHERE id=:id",
                        params!(
                          "timestamp" => now,
                          "id" => last_update.id
                        ),
                    ) {
                        last_update.timestamp = now.to_owned();
                        return Ok(last_update.clone());
                    }
                    return Err(ArmoryFailure::Database("set_character_history".to_owned()));
                }
            }
        } // Else create a new history point and assign it to this character
        self.create_character_history(server_id, update_character_history, character_uid)
    }
}
