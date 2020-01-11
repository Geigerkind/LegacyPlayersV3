use crate::modules::armory::material::Character;
use crate::dto::Failure;
use crate::modules::armory::Armory;

pub trait SetCharacter {
  fn set_character(&self, server_id: u32, character: Character) -> Result<u32, Failure>;
}

impl SetCharacter for Armory {
  fn set_character(&self, server_id: u32, character: Character) -> Result<u32, Failure> {
    Ok(0)
  }
}