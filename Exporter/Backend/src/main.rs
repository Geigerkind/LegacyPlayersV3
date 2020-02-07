#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate mysql_connection;
#[macro_use] extern crate rocket;

mod dto;
mod modules;

use modules::ConsentManager;

fn main() {
  let consent_manager = ConsentManager::default().init();
  rocket::ignite()
    .manage(consent_manager)
    .mount("/API/consent_manager/", routes![
      modules::consent_manager::transfer::character::give_consent,
      modules::consent_manager::transfer::character::withdraw_consent,
      modules::consent_manager::transfer::guild::give_consent,
      modules::consent_manager::transfer::guild::withdraw_consent,
    ])
    .launch();
}
