use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request, State};

use crate::modules::account::Account;
use crate::modules::account::tools::Token;

pub struct Authenticate(pub u32);

impl<'a, 'r> FromRequest<'a, 'r> for Authenticate {
  type Error = ();

  fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
    let auth_header = req.headers().get_one("Authorization");
    if auth_header.is_none() {
      return Failure((Status::Unauthorized, ()));
    }

    let api_token = auth_header.unwrap();
    let account = req.guard::<State<'_, Account>>();
    if account.is_failure() {
      return Failure((Status::Unauthorized, ()));
    }

    let acc_res = account.unwrap();
    let validation = acc_res.validate_token(api_token);
    if validation.is_none() {
      return Failure((Status::Unauthorized, ()));
    }

    Success(Authenticate(validation.unwrap()))
  }
}