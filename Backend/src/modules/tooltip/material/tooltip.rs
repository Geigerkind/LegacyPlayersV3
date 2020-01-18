use mysql_connection::material::MySQLConnection;

#[derive(Debug)]
pub struct Tooltip {
  pub db_main: MySQLConnection,
}

impl Default for Tooltip {
  fn default() -> Self
  {
    Tooltip {
      db_main: MySQLConnection::new("main"),
    }
  }
}

impl Tooltip {
  pub fn init(self) -> Self
  {
    self
  }
}