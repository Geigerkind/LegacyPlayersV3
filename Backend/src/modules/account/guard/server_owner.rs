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

use crate::modules::account::guard::Authenticate;
use crate::modules::data::Data;

pub struct ServerOwner(pub u32);

impl<'a, 'r> FromRequest<'a, 'r> for ServerOwner {
  type Error = ();

  fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
    Authenticate::from_request(req)
      .and_then(|authenticate| {
        let data_req = req.guard::<State<'_, Data>>();
        if data_req.is_failure() {
          return Failure((Status::Unauthorized, ()));
        }

        let data = data_req.unwrap();
        let server_res = data.servers.iter().find(|(_, server)| server.owner.contains(&authenticate.0));
        if server_res.is_none() {
          return Failure((Status::Unauthorized, ()));
        }

        let (id, _) = server_res.unwrap();
        Success(ServerOwner(*id))
      })
  }
}

impl<'a, 'r> OpenApiFromRequest<'a, 'r> for ServerOwner {
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
impl Responder<'static> for ServerOwner {
  fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
    Response::build().status(Status::Unauthorized).ok()
  }
}

impl OpenApiResponder<'static> for ServerOwner {
  fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
    let mut responses = Responses::default();
    let schema = gen.json_schema::<String>();
    add_schema_response(&mut responses, 401, "text/plain", schema)?;
    Ok(responses)
  }
}