#![feature(proc_macro_hygiene, decl_macro)]
extern crate language;
extern crate mail;
#[macro_use]
extern crate mysql_connection;
extern crate okapi;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate str_util;
extern crate time_util;
extern crate validator;

use rocket_okapi::swagger_ui::make_swagger_ui;
use rocket_okapi::swagger_ui::SwaggerUIConfig;
use rocket_okapi::swagger_ui::UrlObject;
use rocket_prometheus::PrometheusMetrics;

use crate::modules::account;
use crate::modules::account::Account;

pub mod dto;
pub mod modules;

fn main() {
  let account: account::Account = account::Account::default();
  Account::init(&account);
  let prometheus = PrometheusMetrics::new();
  let mut igniter = rocket::ignite();
  igniter = igniter.manage(account);
  igniter = igniter.attach(prometheus.clone());
  igniter = igniter.mount("/metrics", prometheus);
  igniter = igniter.mount("/API/",
                          make_swagger_ui(&SwaggerUIConfig {
                            url: None,
                            urls: Some(vec![
                              UrlObject {
                                name: "Account".to_string(),
                                url: "/API/account/openapi.json".to_string(),
                              }
                            ]),
                          }));
  igniter = igniter.mount("/API/account/", routes_with_openapi![
    account::transfer::login::login,
    account::transfer::token::create_token, account::transfer::token::get_tokens, account::transfer::token::delete_token, account::transfer::token::prolong_token,
    account::transfer::delete::request, account::transfer::delete::confirm,
    account::transfer::create::create, account::transfer::create::confirm, account::transfer::create::resend_confirm,
    account::transfer::get::get_account_information,
    account::transfer::forgot::receive_confirmation, account::transfer::forgot::send_confirmation,
    account::transfer::update::request_mail, account::transfer::update::confirm_mail, account::transfer::update::password, account::transfer::update::nickname]);
  igniter.launch();
}