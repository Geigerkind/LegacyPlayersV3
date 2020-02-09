use crate::modules::consent_manager::domain_value::CharacterWithConsent;
use crate::modules::ConsentManager;
use mysql_connection::tools::Select;
use crate::modules::consent_manager::tools::CharacterConsent;

pub trait ManagerFrontend {
  fn get_characters(&self, account_id: u32) -> Vec<CharacterWithConsent>;
}

impl ManagerFrontend for ConsentManager {
  fn get_characters(&self, account_id: u32) -> Vec<CharacterWithConsent> {
    self.db_characters.select_wparams("SELECT guid, name FROM characters \
      WHERE account=:account_id", &|mut row| {
      let guid: u32 = row.take(0).unwrap();
      let name: String = row.take(1).unwrap();
      (guid, name)
    },params!("account_id" => account_id)).iter()
      .map(|(character_id, character_name)| {
        let consent = self.has_given_consent(*character_id);
        CharacterWithConsent {
          character_id: *character_id,
          character_name: character_name.to_owned(),
          consent
        }
      }).collect()
  }
}