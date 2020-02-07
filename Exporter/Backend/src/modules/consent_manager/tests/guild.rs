use crate::modules::ConsentManager;
use crate::modules::consent_manager::tools::GuildConsent;
use mysql_connection::tools::Execute;

#[test]
fn guild_consent_happy() {
  let consent_manager = ConsentManager::default().init();
  let guild_id = 2^30;
  let character_id = 2^30;

  assert!(!consent_manager.has_given_consent(guild_id));
  let result = consent_manager.give_consent(guild_id, character_id);
  assert!(result.is_ok());
  assert!(consent_manager.has_given_consent(guild_id));

  let consent_manager2 = ConsentManager::default().init();
  assert!(consent_manager2.has_given_consent(guild_id));

  let result2 = consent_manager.withdraw_consent(guild_id, character_id);
  assert!(result2.is_ok());
  assert!(!consent_manager.has_given_consent(guild_id));

  let consent_manager3 = ConsentManager::default().init();
  assert!(!consent_manager3.has_given_consent(guild_id));

  // Cleanup
  consent_manager.db_lp_consent.execute_wparams("DELETE FROM guild_consent WHERE guild_id=:guild_id", params!("guild_id" => guild_id));
}

#[test]
fn guild_consent_unhappy() {
  // TODO: Test that if the character_id is not the guild master, it should not be possible to give consent
}