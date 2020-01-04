use std::collections::HashMap;

use mysql_connection::material::MySQLConnection;
use mysql_connection::tools::Select;

use crate::modules::data::domain_value::{Expansion, Language, Localization, Race, Profession, Server};

#[derive(Debug)]
pub struct Data {
  pub db_main: MySQLConnection,
  pub expansions: HashMap<u8, Expansion>,
  pub languages: HashMap<u8, Language>,
  pub localization: Vec<HashMap<u32, String>>,
  pub races: HashMap<u8, Race>,
  pub professions: HashMap<u8, Profession>,
  pub servers: HashMap<u32, Server>
}

impl Default for Data {
  fn default() -> Self
  {
    Data {
      db_main: MySQLConnection::new("main"),
      expansions: HashMap::new(),
      languages: HashMap::new(),
      localization: Vec::new(),
      races: HashMap::new(),
      professions: HashMap::new(),
      servers: HashMap::new()
    }
  }
}

impl Data {
  pub fn init(mut self) -> Self
  {
    self.expansions.init(&self.db_main);
    self.languages.init(&self.db_main);
    for _i in 0..self.languages.len() {
      self.localization.push(HashMap::new());
    }
    self.localization.init(&self.db_main);
    self.races.init(&self.db_main);
    self.professions.init(&self.db_main);
    self.servers.init(&self.db_main);
    self
  }
}


// Initializer for the collections
trait Init {
  fn init(&mut self, db: &MySQLConnection);
}

impl Init for HashMap<u8, Expansion> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_expansion", &|mut row| {
      Expansion {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, Language> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_language", &|mut row| {
      Language {
        id: row.take(0).unwrap(),
        name: row.take(1).unwrap(),
        short_code: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for Vec<HashMap<u32, String>> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_localization ORDER BY language_id, id", &|mut row| {
      Localization {
        language_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        content: row.take(2).unwrap()
      }
    }).iter().for_each(|localization| {
      self.get_mut(localization.language_id as usize - 1).unwrap().insert(localization.id, localization.content.to_owned());
    });
  }
}

impl Init for HashMap<u8, Race> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_race", &|mut row| {
      Race {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, Profession> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_profession", &|mut row| {
      Profession {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u32, Server> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_server", &|mut row| {
      Server {
        id: row.take(0).unwrap(),
        expansion_id: row.take(1).unwrap(),
        name: row.take(2).unwrap()
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}