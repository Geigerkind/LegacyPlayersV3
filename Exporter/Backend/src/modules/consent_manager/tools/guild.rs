use crate::dto::Failure;
use crate::modules::ConsentManager;
use mysql_connection::tools::Execute;
use crate::modules::consent_manager::tools::broadcast::BroadcastConsent;

pub trait GuildConsent {
  fn has_given_consent(&self, guild_id: u32) -> bool;
  fn give_consent(&self, guild_id: u32, character_id: u32) -> Result<(), Failure>;
  fn withdraw_consent(&self, guild_id: u32, character_id: u32) -> Result<(), Failure>;
  fn is_guild_master(&self, character_id: u32, guild_id: u32) -> bool;
}

impl GuildConsent for ConsentManager {
  fn has_given_consent(&self, guild_id: u32) -> bool {
    let guild_consent = self.guild_consent.read().unwrap();
    guild_consent.contains(&guild_id)
  }

  fn give_consent(&self, guild_id: u32, character_id: u32) -> Result<(), Failure> {
    if self.has_given_consent(guild_id) {
      return Err(Failure::ConsentAlreadyGiven);
    }

    if !self.is_guild_master(character_id, guild_id) {
      return Err(Failure::NotTheGuildMaster);
    }

    let mut guild_consent = self.guild_consent.write().unwrap();
    if self.db_lp_consent.execute_wparams("INSERT INTO guild_consent (`guild_id`, `responsible_character_id`) VALUES (:guild_id, :character_id)",
                                          params!(
                                            "guild_id" => guild_id,
                                            "character_id" => character_id
                                          )) {
      guild_consent.insert(guild_id);
      self.broadcast_guild(false, guild_id);
      return Ok(())
    }
    Err(Failure::Database)
  }

  fn withdraw_consent(&self, guild_id: u32, character_id: u32) -> Result<(), Failure> {
    if !self.has_given_consent(guild_id) {
      return Err(Failure::NoConsentGivenYet);
    }

    if !self.is_guild_master(character_id, guild_id) {
      return Err(Failure::NotTheGuildMaster);
    }

    let mut guild_consent = self.guild_consent.write().unwrap();
    if self.db_lp_consent.execute_wparams("UPDATE guild_consent SET consent_withdrawn_when = UNIX_TIMESTAMP() \
                                                   WHERE id = (SELECT MIN(id) FROM guild_consent WHERE ISNULL(consent_withdrawn_when) AND guild_id=:guild_id)",
                                          params!(
                                            "guild_id" => guild_id
                                          )) {
      guild_consent.remove(&guild_id);
      self.broadcast_guild(true, guild_id);
      return Ok(())
    }
    Err(Failure::Database)
  }

  // TODO: Implement!
  fn is_guild_master(&self, character_id: u32, guild_id: u32) -> bool {
    true
  }
}