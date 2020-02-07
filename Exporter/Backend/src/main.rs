#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate mysql_connection;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod dto;
mod modules;

use modules::ConsentManager;

fn main() {
  let consent_manager = ConsentManager::default().init();
  rocket::ignite()
    .manage(consent_manager)
    .mount("/API/consent_manger/", routes![
      modules::consent_manager::transfer::character::give_consent,
      modules::consent_manager::transfer::character::withdraw_consent,
    ])
    .launch();
}
