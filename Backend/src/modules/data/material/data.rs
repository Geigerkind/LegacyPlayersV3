use std::collections::HashMap;

use mysql_connection::material::MySQLConnection;
use mysql_connection::tools::Select;

use crate::modules::data::domain_value::Expansion;

#[derive(Debug)]
pub struct Data {
  pub db_main: MySQLConnection,
  pub expansions: HashMap<u8, Expansion>,
}

impl Default for Data {
  fn default() -> Self
  {
    let mut data = Data {
      db_main: MySQLConnection::new("main"),
      expansions: HashMap::new(),
    };
    Data::init(&mut data);
    data
  }
}

impl Data {
  fn init(&mut self) {
    self.expansions.init(&self.db_main);
  }
}


// Initializer for the collections
trait Init {
  fn init(&mut self, db: &MySQLConnection);
}

impl Init for HashMap<u8, Expansion> {
  fn init(&mut self, db: &MySQLConnection) {
    for expansion in db.select("SELECT * FROM data_expansion", &|mut row| {
      Expansion {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }) {
      self.insert(expansion.id, expansion);
    }
  }
}