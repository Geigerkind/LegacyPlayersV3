use mysql_connection::material::MySQLConnection;
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct ArmoryExporter {
  pub db_characters: MySQLConnection,
  pub sender_character: Option<Sender<String>>
}

impl Default for ArmoryExporter {
  fn default() -> Self {
    ArmoryExporter {
      db_characters: MySQLConnection::new("characters"),
      sender_character: None
    }
  }
}

impl ArmoryExporter {
  pub fn init(self) -> Self
  {
    self
  }
}