use crate::util::database::*;

use crate::modules::armory::{dto::ArmoryFailure, tools::GetCharacter, Armory};
use crate::params;

pub trait DeleteCharacter {
    fn delete_character(&self, db_main: &mut impl Execute, id: u32) -> Result<(), ArmoryFailure>;
    fn delete_character_by_uid(&self, db_main: &mut impl Execute, server_id: u32, uid: u64) -> Result<(), ArmoryFailure>;
}

impl DeleteCharacter for Armory {
    fn delete_character(&self, db_main: &mut impl Execute, id: u32) -> Result<(), ArmoryFailure> {
        let mut characters = self.characters.write().unwrap();
        if db_main.execute_wparams(
            "DELETE FROM armory_character WHERE id=:id",
            params!(
              "id" => id
            ),
        ) {
            // TODO: Correct but inefficient!
            let mut cache = self.cache_char_name_to_id.write().unwrap();
            cache.clear();
            for (char_id, character) in characters.iter().filter(|(_, character)| character.last_update.is_some()) {
                let vec = cache.entry(character.last_update.as_ref().unwrap().character_name.clone()).or_insert_with(Vec::new);
                if !vec.contains(char_id) {
                    vec.push(*char_id);
                }
            }
            return characters.remove(&id).ok_or(ArmoryFailure::InvalidInput).map(|_| ());
        }
        Err(ArmoryFailure::Database("delete_character".to_owned()))
    }

    fn delete_character_by_uid(&self, db_main: &mut impl Execute, server_id: u32, uid: u64) -> Result<(), ArmoryFailure> {
        self.get_character_id_by_uid(server_id, uid).ok_or(ArmoryFailure::InvalidInput).and_then(|id| self.delete_character(db_main, id))
    }
}
