use mysql_connection::tools::{Execute, Select};

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::material::Character;

pub trait CreateCharacter {
  fn create_character(&self, server_id: u32, server_uid: u64) -> Result<u32, Failure>;
}

impl CreateCharacter for Armory {
  fn create_character(&self, server_id: u32, server_uid: u64) -> Result<u32, Failure> {
    let mut characters = self.characters.write().unwrap();
    if self.db_main.execute_wparams("INSERT INTO armory_character (`server_id`, `server_uid`) VALUES (:server_id, :server_uid)", params!(
      "server_id" => server_id,
      "server_uid" => server_uid,
    )) {
      let character_res = self.db_main.select_wparams_value("SELECT id FROM armory_character WHERE server_id=:server_id AND server_uid=:server_uid", &|mut row| {
        Character {
          id: row.take(0).unwrap(),
          server_id,
          server_uid,
          last_update: None,
          history_ids: Vec::new()
        }
      }, params!(
        "server_id" => server_id,
        "server_uid" => server_uid,
      ));
      if character_res.is_some() {
        let character = character_res.unwrap();
        characters.insert(character.id, character);
      }
    }

    Err(Failure::Unknown)
  }
}