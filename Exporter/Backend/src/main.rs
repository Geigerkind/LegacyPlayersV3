#![feature(proc_macro_hygiene, decl_macro)]
extern crate hyper;
#[macro_use]
extern crate mysql_connection;
#[macro_use]
extern crate rocket;

use std::thread;

use modules::ConsentManager;

use crate::modules::{ArmoryExporter, TransportLayer};
use std::sync::mpsc;

mod dto;
mod modules;

pub trait Run {
  fn run(&mut self);
}

fn main() {
  let mut consent_manager = ConsentManager::default();
  let mut transport_layer = TransportLayer::default().init();
  let mut armory_exporter = ArmoryExporter::default().init();

  let (s_char, r_char) = mpsc::channel::<String>();
  let (s_char_consent, r_char_consent) = mpsc::channel::<(bool, u32)>();
  let (s_guild_consent, r_guild_consent) = mpsc::channel::<(bool, u32)>();

  *consent_manager.sender_character_consent.get_mut().unwrap() = Some(s_char_consent.to_owned());
  *consent_manager.sender_guild_consent.get_mut().unwrap() = Some(s_guild_consent.to_owned());
  armory_exporter.sender_character = Some(s_char.to_owned());
  transport_layer.receiver_character_consent = Some(r_char_consent);
  transport_layer.receiver_guild_consent = Some(r_guild_consent);
  transport_layer.receiver_character = Some(r_char);

  thread::spawn(move || transport_layer.run());
  thread::spawn(move || armory_exporter.run());

  rocket::ignite()
    .manage(consent_manager.init())
    .mount("/API/consent_manager/", routes![
      modules::consent_manager::transfer::character::give_consent,
      modules::consent_manager::transfer::character::withdraw_consent,
      modules::consent_manager::transfer::guild::give_consent,
      modules::consent_manager::transfer::guild::withdraw_consent,
    ])
    .launch();
}
