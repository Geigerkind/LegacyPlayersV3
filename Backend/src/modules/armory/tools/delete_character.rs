use crate::dto::Failure;
use crate::modules::armory::Armory;

pub trait DeleteCharacter {
  fn delete_character(&self, id: u32) -> Result<(), Failure>;
}

impl DeleteCharacter for Armory {
  fn delete_character(&self, id: u32) -> Result<(), Failure> {
    unimplemented!()
  }
}