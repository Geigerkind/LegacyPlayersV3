use crate::dto::Failure;
use crate::modules::ConsentManager;

pub trait CharacterConsent {
  fn give_consent(&self, character_id: u32) -> Result<(), Failure>;
  fn withdraw_consent(&self, character_id: u32) -> Result<(), Failure>;
}

impl CharacterConsent for ConsentManager {
  fn give_consent(&self, character_id: u32) -> Result<(), Failure> {
    unimplemented!()
  }

  fn withdraw_consent(&self, character_id: u32) -> Result<(), Failure> {
    unimplemented!()
  }
}