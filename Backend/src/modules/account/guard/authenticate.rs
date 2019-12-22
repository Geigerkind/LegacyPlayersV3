use okapi::openapi3::{Parameter, ParameterValue, Responses};
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request, State};
use rocket::Response;
use rocket::response::Responder;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest};
use rocket_okapi::response::OpenApiResponder;
use rocket_okapi::util::add_schema_response;

use crate::modules::account::Account;
use crate::modules::account::tools::Token;

pub struct Authenticate(pub u32);

impl<'a, 'r> FromRequest<'a, 'r> for Authenticate {
  type Error = ();

  fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
    let auth_header = req.headers().get_one("X-Authorization");
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

impl<'a, 'r> OpenApiFromRequest<'a, 'r> for Authenticate {
  fn request_parameter(_: &mut OpenApiGenerator, _: String) -> rocket_okapi::Result<Parameter> {
    Ok(Parameter {
      name: "X-Authorization".to_owned(),
      location: "header".to_owned(),
      description: None,
      required: true,
      deprecated: false,
      allow_empty_value: false,
      value: ParameterValue::Schema {
        style: None,
        explode: None,
        allow_reserved: false,
        schema: Default::default(),
        example: None,
        examples: None,
      },
      extensions: Default::default(),
    })
  }
}

// This implementation is required from OpenAPI, it does nothing here
// and is not supposed to be used!
impl Responder<'static> for Authenticate {
  fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
    Response::build().status(Status::Unauthorized).ok()
  }
}

impl OpenApiResponder<'static> for Authenticate {
  fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
    let mut responses = Responses::default();
    let schema = gen.json_schema::<String>();
    add_schema_response(&mut responses, 401, "text/plain", schema)?;
    Ok(responses)
  }
}