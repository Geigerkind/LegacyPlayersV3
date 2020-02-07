use std::io::Cursor;

use rocket::{Request, Response};
use rocket::response::Responder;
use rocket::http::Status;

#[derive(Debug)]
pub enum Failure {
  ConsentAlreadyGiven,
  NoConsentGivenYet,
  Database
}

impl Responder<'static> for Failure {
  fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
    let status = match self {
      Failure::ConsentAlreadyGiven => Status::new(520, "ConsentAlreadyGiven"),
      Failure::NoConsentGivenYet => Status::new(521, "NoConsentGivenYet"),
      Failure::Database => Status::new(522, "Database"),
    };
    Response::build()
      .status(status)
      .sized_body(Cursor::new(String::new()))
      .ok()
  }
}