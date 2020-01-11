use std::collections::HashMap;

use mysql_connection::material::MySQLConnection;
use mysql_connection::tools::Select;

#[derive(Debug)]
pub struct Armory {
  pub db_main: MySQLConnection,
}

impl Default for Armory {
  fn default() -> Self
  {
    Armory {
      db_main: MySQLConnection::new("main"),
    }
  }
}

impl Armory {
  pub fn init(mut self) -> Self
  {
    self
  }
}