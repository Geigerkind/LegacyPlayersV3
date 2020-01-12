use mysql_connection::tools::Execute;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::tools::GetCharacter;

pub trait DeleteCharacter {
  fn delete_character(&self, id: u32) -> Result<(), Failure>;
  fn delete_character_by_uid(&self, server_id: u32, uid: u64) -> Result<(), Failure>;
}

impl DeleteCharacter for Armory {
  fn delete_character(&self, id: u32) -> Result<(), Failure> {
    let mut characters = self.characters.write().unwrap();
    if self.db_main.execute_wparams("DELETE FROM armory_character WHERE id=:id", params!(
      "id" => id
    )) {
      return characters.remove(&id).ok_or(Failure::InvalidInput).and_then(|_| Ok(()));
    }
    Err(Failure::Unknown)
  }

  fn delete_character_by_uid(&self, server_id: u32, uid: u64) -> Result<(), Failure> {
    self.get_character_id_by_uid(server_id, uid).ok_or(Failure::InvalidInput).and_then(|id| self.delete_character(id))
  }
}