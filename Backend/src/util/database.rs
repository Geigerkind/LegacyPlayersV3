use crate::mysql::{Value, Row};

#[macro_export]
macro_rules! params {
    ( $( $key:expr => $value:expr ),* ) => {
        vec![
            $(
                ($key.to_string(), crate::mysql::Value::from($value)),
            )*
        ]
    }
}

pub trait Execute {
    fn execute_one(&mut self, query_str: &str) -> bool;
    fn execute_wparams(&mut self, query_str: &str, params: std::vec::Vec<(std::string::String, Value)>) -> bool;
    fn execute_batch_wparams<T>(&mut self, query_str: &str, items: Vec<T>, params: &dyn Fn(&T) -> std::vec::Vec<(std::string::String, Value)>) -> bool;
}

pub trait Exists {
    fn exists(&mut self, query_str: &str) -> bool;
    fn exists_wparams(&mut self, query_str: &str, params: std::vec::Vec<(std::string::String, Value)>) -> bool;
}

use mockall::{automock};
#[automock]
pub trait Select2 {
    fn select_wparams2<T: 'static, F: 'static + (Fn(Row) -> T)>(&mut self, query_str: &str, process_row: F, params: std::vec::Vec<(std::string::String, Value)>) -> Vec<T>;
    fn select_wparams_value2<T: 'static, F: 'static + (Fn(Row) -> T)>(&mut self, query_str: &str, process_row: F, params: std::vec::Vec<(std::string::String, Value)>) -> Option<T>;
}

pub trait Select {
    fn select<T>(&mut self, query_str: &str, process_row: &dyn Fn(Row) -> T) -> Vec<T>;
    fn select_wparams<T>(&mut self, query_str: &str, process_row: &dyn Fn(Row) -> T, params: std::vec::Vec<(std::string::String, Value)>) -> Vec<T>;
    fn select_value<T>(&mut self, query_str: &str, process_row: &dyn Fn(Row) -> T) -> Option<T>;
    fn select_wparams_value<T>(&mut self, query_str: &str, process_row: &dyn Fn(Row) -> T, params: std::vec::Vec<(std::string::String, Value)>) -> Option<T>;
}

impl Execute for crate::mysql::Conn {
    fn execute_one(&mut self, query_str: &str) -> bool {
        self.prep_exec(query_str, ()).is_ok()
    }

    fn execute_wparams(&mut self, query_str: &str, params: std::vec::Vec<(std::string::String, Value)>) -> bool {
        self.prep_exec(query_str, params).is_ok()
    }

    fn execute_batch_wparams<T>(&mut self, _query_str: &str, _items: Vec<T>, _params: &dyn Fn(&T) -> std::vec::Vec<(std::string::String, Value)>) -> bool {
        unimplemented!()
        //self.exec_batch(query_str, items.iter().map(params)).is_ok()
    }
}

impl Exists for crate::mysql::Conn {
    fn exists(&mut self, query_str: &str) -> bool {
        self.select_value(&["SELECT EXISTS(", query_str, ")"].concat(), &|row| crate::mysql::from_row(row)).unwrap()
    }

    fn exists_wparams(&mut self, query_str: &str, params: Vec<(String, Value)>) -> bool {
        self.select_wparams_value(&["SELECT EXISTS(", query_str, ")"].concat(), &|row| crate::mysql::from_row(row), params).unwrap()
    }
}

impl Select for crate::mysql::Conn {
    fn select<T>(&mut self, query_str: &str, process_row: &dyn Fn(Row) -> T) -> Vec<T> {
        self.prep_exec(query_str, ())
            .map(|result| {
                result.map(|x| x.unwrap())
                    .map(|row| process_row(crate::mysql::from_row(row)))
                    .collect()
            }).unwrap()
    }

    fn select_wparams<T>(&mut self, query_str: &str, process_row: &dyn Fn(Row) -> T, params: Vec<(String, Value)>) -> Vec<T> {
        self.prep_exec(query_str, params)
            .map(|result| {
                result.map(|x| x.unwrap())
                    .map(|row| process_row(crate::mysql::from_row(row)))
                    .collect()
            }).unwrap()
    }

    fn select_value<T>(&mut self, query_str: &str, process_row: &dyn Fn(Row) -> T) -> Option<T> {
        self.select(query_str, process_row).pop()
    }

    fn select_wparams_value<T>(&mut self, query_str: &str, process_row: &dyn Fn(Row) -> T, params: Vec<(String, Value)>) -> Option<T> {
        self.select_wparams(query_str, process_row, params).pop()
    }
}

impl Select2 for crate::mysql::Conn {
    fn select_wparams2<T: 'static, F: 'static + (Fn(Row) -> T)>(&mut self, query_str: &str, process_row: F, params: Vec<(String, Value)>) -> Vec<T> {
        self.prep_exec(query_str, params)
            .map(|result| {
                result.map(|x| x.unwrap())
                    .map(|row| process_row(crate::mysql::from_row(row)))
                    .collect()
            }).unwrap()
    }

    fn select_wparams_value2<T: 'static, F: 'static + (Fn(Row) -> T)>(&mut self, query_str: &str, process_row: F, params: Vec<(String, Value)>) -> Option<T> {
        self.select_wparams2(query_str, process_row, params).pop()
    }
}