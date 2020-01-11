use std::collections::HashMap;

use mysql_connection::material::MySQLConnection;
use mysql_connection::tools::Select;

use crate::modules::armory::material::Character;

#[derive(Debug)]
pub struct Armory {
  pub db_main: MySQLConnection,
  characters: HashMap<u32, Character>,
}

impl Default for Armory {
  fn default() -> Self
  {
    Armory {
      db_main: MySQLConnection::new("main"),
      characters: HashMap::new(),
    }
  }
}

impl Armory {
  pub fn init(mut self) -> Self
  {


    self
  }
}