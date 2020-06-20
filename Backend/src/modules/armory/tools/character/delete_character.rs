use crate::util::database::*;

use crate::modules::armory::{dto::ArmoryFailure, tools::GetCharacter, Armory};
use crate::params;

pub trait DeleteCharacter {
    fn delete_character(&self, db_main: &mut crate::mysql::Conn, id: u32) -> Result<(), ArmoryFailure>;
    fn delete_character_by_uid(&self, db_main: &mut crate::mysql::Conn, server_id: u32, uid: u64) -> Result<(), ArmoryFailure>;
}

impl DeleteCharacter for Armory {
    fn delete_character(&self, db_main: &mut crate::mysql::Conn, id: u32) -> Result<(), ArmoryFailure> {
        let mut characters = self.characters.write().unwrap();
        if db_main.execute_wparams(
            "DELETE FROM armory_character WHERE id=:id",
            params!(
              "id" => id
            ),
        ) {
            return characters.remove(&id).ok_or(ArmoryFailure::InvalidInput).map(|_| ());
        }
        Err(ArmoryFailure::Database("delete_character".to_owned()))
    }

    fn delete_character_by_uid(&self, db_main: &mut crate::mysql::Conn, server_id: u32, uid: u64) -> Result<(), ArmoryFailure> {
        self.get_character_id_by_uid(server_id, uid).ok_or(ArmoryFailure::InvalidInput).and_then(|id| self.delete_character(db_main, id))
    }
}
