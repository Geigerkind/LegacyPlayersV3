use crate::modules::ConsentManager;
use crate::modules::consent_manager::tools::CharacterConsent;
use mysql_connection::tools::Execute;

#[test]
fn character_consent() {
  let consent_manager = ConsentManager::default().init();
  let character_id = 2^30;

  assert!(!consent_manager.has_given_consent(character_id));
  let result = consent_manager.give_consent(character_id);
  assert!(result.is_ok());
  assert!(consent_manager.has_given_consent(character_id));

  let consent_manager2 = ConsentManager::default().init();
  assert!(consent_manager2.has_given_consent(character_id));

  let result2 = consent_manager.withdraw_consent(character_id);
  assert!(result2.is_ok());
  assert!(!consent_manager.has_given_consent(character_id));

  let consent_manager3 = ConsentManager::default().init();
  assert!(!consent_manager3.has_given_consent(character_id));

  // Cleanup
  consent_manager.db_lp_consent.execute_wparams("DELETE FROM character_consent WHERE character_id=:character_id", params!("character_id" => character_id));
}