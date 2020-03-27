#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate mysql_connection;
#[macro_use]
extern crate rocket;
extern crate time_util;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;

use std::{thread, env};
use dotenv::dotenv;

use modules::ConsentManager;

use crate::modules::{ArmoryExporter, TransportLayer, CharacterDto, ServerExporter};
use std::sync::mpsc;

mod dto;
mod modules;

pub trait Run {
  fn run(&mut self);
}

#[derive(Debug, Serialize)]
struct ProlongToken {
  token: String,
  days: u8
}

fn prolong_token() {
  let token = env::var("LP_API_TOKEN").unwrap();
  let uri = env::var("URL_PROLONG_TOKEN").unwrap();
  let client = reqwest::blocking::Client::new();
  let _ = client
    .post(&uri)
    .body(serde_json::to_string(&ProlongToken {
      token,
      days: 30
    }).unwrap())
    .send();
}

fn main() {
  dotenv().ok();

  prolong_token();

  let mut consent_manager = ConsentManager::default();
  let mut transport_layer = TransportLayer::default().init();
  let mut armory_exporter = ArmoryExporter::default().init();
  let mut server_exporter = ServerExporter::default().init();

  let (s_char, r_char) = mpsc::channel::<(u32, CharacterDto)>();
  let (s_char_consent, r_char_consent) = mpsc::channel::<(bool, u32)>();
  let (s_guild_consent, r_guild_consent) = mpsc::channel::<(bool, u32)>();
  let (s_server_msg, r_server_msg) = mpsc::channel::<(Vec<u32>, String)>();

  *consent_manager.sender_character_consent.get_mut().unwrap() = Some(s_char_consent.to_owned());
  *consent_manager.sender_guild_consent.get_mut().unwrap() = Some(s_guild_consent.to_owned());
  armory_exporter.sender_character = Some(s_char.to_owned());
  server_exporter.sender_message = Some(s_server_msg);
  transport_layer.receiver_character_consent = Some(r_char_consent);
  transport_layer.receiver_guild_consent = Some(r_guild_consent);
  transport_layer.receiver_character = Some(r_char);
  transport_layer.receiver_server_message = Some(r_server_msg);

  thread::spawn(move || transport_layer.run());
  thread::spawn(move || armory_exporter.run());
  thread::spawn(move || server_exporter.run());

  rocket::ignite()
    .manage(consent_manager.init())
    .mount("/API/consent_manager/", routes![
      modules::consent_manager::transfer::character::get_characters,
      modules::consent_manager::transfer::character::give_consent,
      modules::consent_manager::transfer::character::withdraw_consent,
      modules::consent_manager::transfer::guild::give_consent,
      modules::consent_manager::transfer::guild::withdraw_consent,
    ])
    .launch();
}
