use mysql;

use crate::material::MySQLConnection;

pub trait Select {
  fn select<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Vec<T>;
  fn select_wparams<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Vec<T>;
  fn select_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Option<T>;
  fn select_wparams_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Option<T>;
}

impl Select for MySQLConnection {
  fn select<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Vec<T>
  {
    self.con.prep_exec(query_str, ())
      .map(|result| {
        result.map(|x| x.unwrap())
          .map(|row| process_row(row))
          .collect()
      }).unwrap()
  }

  fn select_wparams<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Vec<T>
  {
    self.con.prep_exec(query_str, params)
      .map(|result| {
        result.map(|x| x.unwrap())
          .map(|row| process_row(row))
          .collect()
      }).unwrap()
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