use crate::modules::armory::material::CharacterHistory;
use crate::modules::armory::Armory;
use crate::dto::Failure;
use mysql_connection::tools::Select;
use crate::modules::armory::tools::GetCharacterInfo;
use crate::modules::armory::domain_value::CharacterGuild;

pub trait GetCharacterHistory {
  fn get_character_history(&self, character_history_id: u32) -> Result<CharacterHistory, Failure>;
}

impl GetCharacterHistory for Armory {
  fn get_character_history(&self, character_history_id: u32) -> Result<CharacterHistory, Failure> {
    let character_history = self.db_main.select_wparams_value("SELECT * FROM armory_character_history WHERE id=:id", &|mut row| {
      let character_info = self.get_character_info(row.take(2).unwrap());
      if character_info.is_err() {
        return Err(character_info.err().unwrap());
      }

      Ok(CharacterHistory {
        id: character_history_id,
        character_id: row.take(1).unwrap(),
        character_info: character_info.unwrap(),
        character_name: row.take(3).unwrap(),
        character_guild: row.take_opt(4).unwrap().ok().and_then(|guild_id| Some(CharacterGuild {
          guild_id,
          rank: row.take(5).unwrap()
        })),
        character_title: row.take_opt(6).unwrap().ok(),
        timestamp: row.take(7).unwrap()
      })
    }, params!(
      "id" => character_history_id
    ));

    if character_history.is_none() {
      return Err(Failure::Unknown);
    }
    character_history.unwrap()
  }
}