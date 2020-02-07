use std::io::Read;

use hyper::Client;
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request, State};
use rocket::Response;
use rocket::response::Responder;

pub struct Authenticate;

impl<'a, 'r> FromRequest<'a, 'r> for Authenticate {
  type Error = ();

  fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
    let auth_header = req.headers().get_one("X-Authorization");
    if auth_header.is_none() {
      return Failure((Status::Unauthorized, ()));
    }
    let split: Vec<&str> = auth_header.unwrap().split(",").collect();
    if split.len() != 2 {
      return Failure((Status::Unauthorized, ()));
    }
    let token = split[0].to_string();
    let account_id_res = split[1].parse::<u32>();
    if account_id_res.is_err() {
      return Failure((Status::Unauthorized, ()));
    }
    let account_id = account_id_res.unwrap();

    let client = Client::new();
    let uri = format!("http://localhost:8001/token_validator/{}/{}", token, account_id);
    let resp = client.get(&uri).send();
    let mut buffer: [u8; 4] = [0; 4];
    let result = resp.unwrap().read(&mut buffer);
    if result.is_err() || (buffer != [116, 114, 117, 101] && buffer != [49, 0, 0, 0]) {
      return Failure((Status::Unauthorized, ()));
    }

    Success(Authenticate)
  }
}