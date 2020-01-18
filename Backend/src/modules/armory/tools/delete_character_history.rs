use crate::dto::Failure;
use crate::modules::armory::Armory;
use mysql_connection::tools::Execute;
use crate::modules::armory::tools::GetCharacterHistory;

pub trait DeleteCharacterHistory {
  fn delete_character_history(&self, character_history_id: u32) -> Result<(), Failure>;
}

impl DeleteCharacterHistory for Armory {
  fn delete_character_history(&self, character_history_id: u32) -> Result<(), Failure> {
    let character_history_res = self.get_character_history(character_history_id);
    if character_history_res.is_err() {
      return Err(Failure::InvalidInput);
    }
    let character_history = character_history_res.unwrap();

    let mut characters = self.characters.write().unwrap();
    if self.db_main.execute_wparams("DELETE FROM armory_character_history WHERE id=:id", params!(
      "id" => character_history_id
    )) {
      let mut character = characters.get_mut(&character_history.character_id).unwrap();
      character.history_ids.remove_item(&character_history_id);
      if character.last_update.contains(&character_history) {
        let last_id = character.history_ids.last();
        if last_id.is_some() {
          character.last_update = self.get_character_history(last_id.unwrap().clone()).ok();
        }
      }
      return Ok(());
    }
    Err(Failure::Unknown)
  }
}