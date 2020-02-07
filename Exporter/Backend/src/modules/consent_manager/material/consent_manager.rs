use std::collections::BTreeSet;
use std::sync::RwLock;

use mysql_connection::material::MySQLConnection;
use mysql_connection::tools::Select;

#[derive(Debug)]
pub struct ConsentManager {
  pub db_lp_consent: MySQLConnection,
  pub character_consent: RwLock<BTreeSet<u32>>,
  pub guild_consent: RwLock<BTreeSet<u32>>,
}

impl Default for ConsentManager {
  fn default() -> Self {
    ConsentManager {
      db_lp_consent: MySQLConnection::new("lp_consent"),
      character_consent: RwLock::new(BTreeSet::new()),
      guild_consent: RwLock::new(BTreeSet::new()),
    }
  }
}

impl ConsentManager {
  pub fn init(self) -> Self
  {
    {
      let mut character_consent = self.character_consent.write().unwrap();
      let mut guild_consent = self.guild_consent.write().unwrap();

      self.db_lp_consent.select("SELECT character_id FROM character_consent WHERE ISNULL(consent_withdrawn_when)", &|mut row| {
        let character_id: u32 = row.take(0).unwrap();
        character_id
      }).iter().for_each(|result| {
        if !character_consent.contains(result) {
          character_consent.insert(*result);
        }
      });

      self.db_lp_consent.select("SELECT guild_id FROM guild_consent WHERE ISNULL(consent_withdrawn_when)", &|mut row| {
        let guild_id: u32 = row.take(0).unwrap();
        guild_id
      }).iter().for_each(|result| {
        if !guild_consent.contains(result) {
          guild_consent.insert(*result);
        }
      });
    }
    self
  }
}