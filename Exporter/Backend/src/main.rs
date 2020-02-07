#![feature(proc_macro_hygiene)]
#[macro_use] extern crate mysql_connection;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

fn main() {
    println!("Hello, world!");
}
