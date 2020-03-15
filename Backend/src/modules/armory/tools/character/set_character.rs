use crate::dto::CheckPlausability;
use crate::modules::armory::dto::{ArmoryFailure, CharacterDto};
use crate::modules::armory::material::Character;
use crate::modules::armory::tools::{CreateCharacter, GetCharacter, SetCharacterHistory};
use crate::modules::armory::Armory;

pub trait SetCharacter {
    fn set_character(
        &self,
        server_id: u32,
        update_character: CharacterDto,
    ) -> Result<Character, ArmoryFailure>;
}

impl SetCharacter for Armory {
    fn set_character(
        &self,
        server_id: u32,
        update_character: CharacterDto,
    ) -> Result<Character, ArmoryFailure> {
        // Validation
        if !update_character.is_plausible() {
            return Err(ArmoryFailure::ImplausibleInput);
        }

        // Create the character if necessary
        let character_id_res = self.create_character(server_id, update_character.server_uid);
        if character_id_res.is_err() {
            return Err(character_id_res.err().unwrap());
        }
        let character_id = character_id_res.unwrap();

        // Set the character history
        if update_character.character_history.is_some() {
            let character_history_res = self.set_character_history(
                server_id,
                update_character.character_history.unwrap(),
                update_character.server_uid,
            );
            if character_history_res.is_err() {
                return Err(character_history_res.err().unwrap());
            }
        }

        self.get_character(character_id)
            .ok_or_else(|| ArmoryFailure::Database("get_character".to_owned()))
    }
}
