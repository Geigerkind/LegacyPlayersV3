use mysql;

use crate::material::MySQLConnection;
use mysql::prelude::Queryable;

pub trait Execute {
    fn execute(&self, query_str: &str) -> bool;
    fn execute_wparams(&self, query_str: &str, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> bool;
    fn execute_batch_wparams<T>(&self, query_str: &str, items: Vec<T>, process_row: &dyn Fn(&T) -> std::vec::Vec<(std::string::String, mysql::Value)>) -> bool;
}

impl Execute for MySQLConnection {
    fn execute(&self, query_str: &str) -> bool {
        self.con.get_conn().unwrap().exec_drop(query_str, ()).is_ok()
    }

    fn execute_wparams(&self, query_str: &str, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> bool {
        self.con.get_conn().unwrap().exec_drop(query_str, params).is_ok()
    }

    fn execute_batch_wparams<T>(&self, query_str: &str, items: Vec<T>, params: &dyn Fn(&T) -> std::vec::Vec<(std::string::String, mysql::Value)>) -> bool {
        self.con.get_conn().unwrap().exec_batch(query_str, items.iter().map(params)).is_ok()
    }
}
