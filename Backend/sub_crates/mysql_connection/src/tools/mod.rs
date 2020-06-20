mod execute;
mod exists;
mod select;

pub trait Execute {
  fn execute(&self, query_str: &str) -> bool;
  fn execute_wparams(&self, query_str: &str, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> bool;
  fn execute_batch_wparams<T>(&self, query_str: &str, items: Vec<T>, process_row: &dyn Fn(&T) -> std::vec::Vec<(std::string::String, mysql::Value)>) -> bool;
}

pub trait Exists {
  fn exists(&self, query_str: &str) -> bool;
  fn exists_wparams(&self, query_str: &str, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> bool;
}

pub trait Select {
  fn select<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Vec<T>;
  fn select_wparams<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Vec<T>;
  fn select_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Option<T>;
  fn select_wparams_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Option<T>;
}