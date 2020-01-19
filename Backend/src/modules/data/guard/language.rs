use okapi::openapi3::{Parameter, ParameterValue, Responses};
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request, State};
use rocket::Response;
use rocket::response::Responder;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::OpenApiFromRequest;
use rocket_okapi::response::OpenApiResponder;
use rocket_okapi::util::add_schema_response;

use crate::modules::data::Data;
use crate::modules::data::tools::RetrieveLanguage;

pub struct Language(pub u8);

impl<'a, 'r> FromRequest<'a, 'r> for Language {
  type Error = ();

  fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
    let lang_header = req.headers().get_one("X-Language");
    if lang_header.is_none() {
      return Success(Language(1));
    }

    let lang_short_code = lang_header.unwrap().to_lowercase();
    let data_res = req.guard::<State<'_, Data>>();
    if data_res.is_failure() {
      return Success(Language(1));
    }

    let data = data_res.unwrap();
    let language = data.get_language_by_short_code(lang_short_code).and_then(|language| Some(language.id));
    if language.is_none() {
      return Success(Language(1));
    }

    Success(Language(language.unwrap()))
  }
}

impl<'a, 'r> OpenApiFromRequest<'a, 'r> for Language {
  fn request_parameter(_: &mut OpenApiGenerator, _: String) -> rocket_okapi::Result<Parameter> {
    Ok(Parameter {
      name: "X-Language".to_owned(),
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
impl Responder<'static> for Language {
  fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
    Response::build().status(Status::Ok).ok()
  }
}

impl OpenApiResponder<'static> for Language {
  fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
    Ok(Responses::default())
  }
}