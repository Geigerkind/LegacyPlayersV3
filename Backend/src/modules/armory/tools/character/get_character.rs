use crate::modules::armory::material::Character;
use crate::modules::armory::Armory;

pub trait GetCharacter {
    fn get_character_id_by_uid(&self, server_id: u32, uid: u64) -> Option<u32>;
    fn get_character_by_uid(&self, server_id: u32, uid: u64) -> Option<Character>;
    fn get_character(&self, character_id: u32) -> Option<Character>;
    fn get_characters_by_name(&self, character_name: String) -> Vec<Character>;
    fn get_character_by_name(&self, server_id: u32, character_name: String) -> Option<Character>;
}

impl GetCharacter for Armory {
    fn get_character_id_by_uid(&self, server_id: u32, uid: u64) -> Option<u32> {
        let characters = self.characters.read().unwrap();
        characters
            .iter()
            .find(|(_, character)| character.server_id == server_id && character.server_uid == uid)
            .map(|(id, _)| *id)
    }

    fn get_character_by_uid(&self, server_id: u32, uid: u64) -> Option<Character> {
        self.get_character_id_by_uid(server_id, uid)
            .and_then(|character_id| self.get_character(character_id))
    }

    fn get_character(&self, character_id: u32) -> Option<Character> {
        let characters = self.characters.read().unwrap();
        characters
            .get(&character_id)
            .cloned()
    }

    fn get_characters_by_name(&self, character_name: String) -> Vec<Character> {
        let characters = self.characters.read().unwrap();
        let name = character_name.to_lowercase();
        characters
            .iter()
            .filter(|(_, character)| {
                character.last_update.is_some()
                    && character
                        .last_update
                        .as_ref()
                        .unwrap()
                        .character_name
                        .to_lowercase()
                        .contains(&name)
            })
            .map(|(_, character)| character.clone())
            .collect()
    }

    fn get_character_by_name(&self, server_id: u32, character_name: String) -> Option<Character> {
        self.get_characters_by_name(character_name.clone())
            .iter()
            .find(|character| {
                character.server_id == server_id
                    && character
                        .last_update
                        .as_ref()
                        .unwrap()
                        .character_name
                        .to_lowercase()
                        == character_name.to_lowercase()
            })
            .cloned()
    }
}
