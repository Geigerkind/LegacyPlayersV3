pub use self::mysql_connection::MySQLConnection;
mod mysql_connection;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait DbConnection: Send + Sync + 'static {
  fn new(dns: &str) -> Self;
}