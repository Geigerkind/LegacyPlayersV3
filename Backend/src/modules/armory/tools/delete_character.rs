use crate::dto::Failure;
use crate::modules::armory::Armory;
use mysql_connection::tools::Execute;

pub trait DeleteCharacter {
  fn delete_character(&self, id: u32) -> Result<(), Failure>;
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
}