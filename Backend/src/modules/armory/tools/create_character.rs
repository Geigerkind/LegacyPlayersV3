use crate::dto::Failure;
use crate::modules::armory::Armory;

pub trait CreateCharacter {
  fn create_character(&self, server_id: u32) -> Result<(), Failure>;
}

impl CreateCharacter for Armory {
  fn create_character(&self, server_id: u32) -> Result<(), Failure> {

    unimplemented!()
  }
}