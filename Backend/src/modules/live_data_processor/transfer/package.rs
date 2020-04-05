use rocket::{State, Request, Data};
use rocket_contrib::json::Json;
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::modules::account::guard::ServerOwner;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::tools::ProcessMessages;
use rocket::request::Form;
use rocket::data::{FromData, FromDataSimple, Transformed, Outcome, Transform};

#[derive(Debug)]
pub struct Payload<'a> {
  pub payload: &'a [u8]
}

impl<'a> FromData<'a> for Payload<'a> {
  type Error = ();
  type Owned = ();
  type Borrowed = ();

  fn transform(request: &Request<'r>, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
    unimplemented!()
  }

  fn from_data(request: &Request<'r>, outcome: Transformed<'a, Self>) -> Outcome<Self, Self::Error> {
    unimplemented!()
  }
}

#[openapi(skip)]
#[post("/package", format = "multipart/form-data", data = "<params>")]
pub fn get_package(me: State<LiveDataProcessor>, owner: ServerOwner, params: Payload) -> Result<(), LiveDataProcessorFailure> {
  println!("{:?}", params);
  me.process_messages(owner.0, Vec::new())
}