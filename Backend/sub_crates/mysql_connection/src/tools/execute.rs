use mysql;

use crate::material::MySQLConnection;

pub trait Execute {
  fn execute(&self, query_str: &str) -> bool;
  fn execute_wparams(&self, query_str: &str, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> bool;
}

impl Execute for MySQLConnection {
  fn execute(&self, query_str: &str) -> bool
  {
    self.con.prep_exec(query_str, ()).is_ok()
  }

  fn execute_wparams(&self, query_str: &str, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> bool
  {
    self.con.prep_exec(query_str, params).is_ok()
  }
}