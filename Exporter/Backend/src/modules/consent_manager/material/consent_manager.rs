use mysql_connection::material::MySQLConnection;
use std::collections::BTreeSet;
use mysql_connection::tools::Select;

#[derive(Debug)]
pub struct ConsentManager {
  pub db_lp_consent: MySQLConnection,
  pub character_consent: BTreeSet<u32>,
  pub guild_consent: BTreeSet<u32>
}

impl Default for ConsentManager {
  fn default() -> Self {
    ConsentManager {
      db_lp_consent: MySQLConnection::new("lp_consent"),
      character_consent: BTreeSet::new(),
      guild_consent: BTreeSet::new(),
    }
  }
}

impl ConsentManager {
  pub fn init(mut self) -> Self
  {
    self.db_lp_consent.select("SELECT character_id FROM character_consent WHERE ISNULL(consent_withdrawn_when)", &|mut row| {
      let character_id: u32 = row.take(0).unwrap();
      character_id
    }).iter().for_each(|result| {
      if !self.character_consent.contains(result) {
        self.character_consent.insert(*result);
      }
    });

    self.db_lp_consent.select("SELECT guild_id FROM guild_consent WHERE ISNULL(consent_withdrawn_when)", &|mut row| {
      let guild_id: u32 = row.take(0).unwrap();
      guild_id
    }).iter().for_each(|result| {
      if !self.guild_consent.contains(result) {
        self.guild_consent.insert(*result);
      }
    });

    self
  }
}