use mysql;

use crate::material::MySQLConnection;
use mysql::prelude::Queryable;
use mysql::prelude::WithParams;
use mysql::prelude::BinQuery;

pub trait Select {
  fn select<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Vec<T>;
  fn select_wparams<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Vec<T>;
  fn select_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Option<T>;
  fn select_wparams_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Option<T>;
}

impl Select for MySQLConnection {
  fn select<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Vec<T>
  {
    self.con.get_conn().unwrap().query_map(query_str, process_row).unwrap()
  }

  fn select_wparams<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Vec<T>
  {
    query_str.with(params).map(self.con.get_conn().unwrap(), process_row).unwrap()
  }

  fn select_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Option<T>
  {
    self.select(query_str, process_row).pop()
  }

  fn select_wparams_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Option<T>
  {
    self.select_wparams(query_str, process_row, params).pop()
  }
}