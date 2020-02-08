use crate::dto::Failure;
use crate::modules::ConsentManager;
use mysql_connection::tools::Execute;
use crate::modules::consent_manager::tools::broadcast::BroadcastConsent;

pub trait CharacterConsent {
  fn has_given_consent(&self, character_id: u32) -> bool;
  fn give_consent(&self, character_id: u32) -> Result<(), Failure>;
  fn withdraw_consent(&self, character_id: u32) -> Result<(), Failure>;
}

impl CharacterConsent for ConsentManager {
  fn has_given_consent(&self, character_id: u32) -> bool {
    let character_consent = self.character_consent.read().unwrap();
    character_consent.contains(&character_id)
  }

  fn give_consent(&self, character_id: u32) -> Result<(), Failure> {
    if self.has_given_consent(character_id) {
      return Err(Failure::ConsentAlreadyGiven);
    }

    let mut character_consent = self.character_consent.write().unwrap();
    if self.db_lp_consent.execute_wparams("INSERT INTO character_consent (`character_id`) VALUES (:character_id)",
                                          params!(
                                            "character_id" => character_id
                                          )) {
      character_consent.insert(character_id);
      self.broadcast_character(false, character_id);
      return Ok(())
    }
    Err(Failure::Database)
  }

  fn withdraw_consent(&self, character_id: u32) -> Result<(), Failure> {
    if !self.has_given_consent(character_id) {
      return Err(Failure::NoConsentGivenYet);
    }

    let mut character_consent = self.character_consent.write().unwrap();
    if self.db_lp_consent.execute_wparams("UPDATE character_consent SET consent_withdrawn_when = UNIX_TIMESTAMP() \
                                                   WHERE id = (SELECT MIN(id) FROM character_consent WHERE ISNULL(consent_withdrawn_when) AND character_id=:character_id)",
                                          params!(
                                            "character_id" => character_id
                                          )) {
      character_consent.remove(&character_id);
      self.broadcast_character(true, character_id);
      return Ok(())
    }
    Err(Failure::Database)
  }
}