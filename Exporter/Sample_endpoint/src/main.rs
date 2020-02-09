#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;

#[get("/<token>/<account_id>")]
fn validate_token(token: String, account_id: u32) -> Json<bool> {
  Json(token == "abc" && account_id == 5)
}

fn main() {
  rocket::ignite()
    .mount("/token_validator/", routes![
      validate_token
    ])
    .launch();
}
