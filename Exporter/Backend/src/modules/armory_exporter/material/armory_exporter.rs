use std::sync::mpsc::Sender;

use mysql_connection::material::MySQLConnection;

use crate::modules::CharacterDto;
use mysql_connection::tools::Select;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ArmoryExporter {
  pub db_characters: MySQLConnection,
  pub db_lp_consent: MySQLConnection,
  pub sender_character: Option<Sender<(u32, CharacterDto)>>,
  pub last_fetch_time: u64,
  pub gem_enchant_id_to_item_id: HashMap<u32, u32>
}

impl Default for ArmoryExporter {
  fn default() -> Self {
    ArmoryExporter {
      db_characters: MySQLConnection::new("characters"),
      db_lp_consent: MySQLConnection::new("lp_consent"),
      sender_character: None,
      last_fetch_time: 0,
      gem_enchant_id_to_item_id: HashMap::new()
    }
  }
}

impl ArmoryExporter {
  pub fn init(mut self) -> Self
  {
    let expansion_id = 2; // TODO: Env

    self.last_fetch_time = self.db_lp_consent.select_value("SELECT last_fetch FROM meta_data", &|mut row| {
      let last_fetch: u64 = row.take(0).unwrap();
      last_fetch
    }).unwrap();

    self.db_lp_consent.select_wparams("SELECT enchant_id, item_id FROM data_gem \
      WHERE expansion_id=:expansion_id", &|mut row| {
        let enchant_id: u32 = row.take(0).unwrap();
        let item_id: u32 = row.take(1).unwrap();
      (enchant_id, item_id)
    },params!("expansion_id" => expansion_id)).iter()
      .for_each(|(enchant_id, item_id)| { self.gem_enchant_id_to_item_id.insert(*enchant_id, *item_id); });
    self
  }
}