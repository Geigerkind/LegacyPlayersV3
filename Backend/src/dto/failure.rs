use std::io::Cursor;

use okapi::openapi3::Responses;
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponder;
use rocket_okapi::util::add_schema_response;
use schemars::JsonSchema;

#[derive(Debug, JsonSchema)]
pub enum Failure {
  InvalidCredentials,
  InvalidMail,
  InvalidNickname,
  PwnedPassword(u64),
  PasswordTooShort,
  MailIsInUse,
  NicknameIsInUse,
  InvalidUrl,
  MailSend,
  DeleteNotIssued,
  ForgotNotIssued,
  TooManyDays,
  DateInThePast,
  TokenPurposeLength,
  Unknown,
}

impl Responder<'static> for Failure {
  fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
    let mut body: String = String::new();
    let status = match self {
      Failure::InvalidCredentials => Status::new(520, ""),
      Failure::InvalidMail => Status::new(521, ""),
      Failure::InvalidNickname => Status::new(522, ""),
      Failure::PwnedPassword(timed_pwned) => {
        body = timed_pwned.to_string();
        Status::new(523, "")
      }
      Failure::PasswordTooShort => Status::new(524, ""),
      Failure::MailIsInUse => Status::new(525, ""),
      Failure::NicknameIsInUse => Status::new(526, ""),
      Failure::InvalidUrl => Status::new(527, ""),
      Failure::MailSend => Status::new(528, ""),
      Failure::DeleteNotIssued => Status::new(529, ""),
      Failure::ForgotNotIssued => Status::new(530, ""),
      Failure::TooManyDays => Status::new(531, ""),
      Failure::DateInThePast => Status::new(532, ""),
      Failure::TokenPurposeLength => Status::new(533, ""),
      Failure::Unknown => Status::new(599, ""),
    };
    Response::build()
      .status(status)
      .sized_body(Cursor::new(body))
      .ok()
  }
}

impl OpenApiResponder<'static> for Failure {
  fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
    let mut responses = Responses::default();
    let schema = gen.json_schema::<String>();
    add_schema_response(&mut responses, 520, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 521, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 522, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 523, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 524, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 525, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 526, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 527, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 528, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 529, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 530, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 531, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 532, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 533, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 599, "text/plain", schema.clone())?;
    Ok(responses)
  }
}