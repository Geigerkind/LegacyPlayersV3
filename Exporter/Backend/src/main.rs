#![feature(proc_macro_hygiene)]
#[macro_use] extern crate mysql_connection;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod modules;

use modules::ConsentManager;

fn main() {
  let consent_manager = ConsentManager::default().init();
  rocket::ignite()
    .manage(consent_manager)
    .mount("/API/consent_manger/", routes![])
    .launch();
}
