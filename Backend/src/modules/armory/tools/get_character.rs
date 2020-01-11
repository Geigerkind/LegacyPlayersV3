use crate::modules::armory::Armory;
use crate::modules::armory::material::Character;

pub trait GetCharacter {
  fn get_character_id_by_uid(&self, server_id: u32, uid: u64) -> Option<u32>;
  fn get_character_by_uid(&self, server_id: u32, uid: u64) -> Option<Character>;
  fn get_character(&self, character_id: u32) -> Option<Character>;
}

impl GetCharacter for Armory {
  fn get_character_id_by_uid(&self, server_id: u32, uid: u64) -> Option<u32> {
    let characters = self.characters.read().unwrap();
    characters.iter().find(|(_, character)| character.server_id == server_id && character.server_uid == uid)
      .and_then(|(id, _)| Some(id.clone()))
  }

  fn get_character_by_uid(&self, server_id: u32, uid: u64) -> Option<Character> {
    self.get_character_id_by_uid(server_id, uid)
      .and_then(|character_id| self.get_character(character_id))
  }

  fn get_character(&self, character_id: u32) -> Option<Character> {
    let characters = self.characters.read().unwrap();
    characters.get(&character_id)
      .and_then(|character| Some(character.clone()))
  }
}