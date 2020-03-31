use rocket::State;
use rocket_contrib::json::Json;
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::modules::account::guard::ServerOwner;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;

#[openapi]
#[post("/package", format = "application/json", data = "<params>")]
pub fn get_package(me: State<LiveDataProcessor>, owner: ServerOwner, params: Json<Vec<String>>) -> Result<(), LiveDataProcessorFailure> {
  unimplemented!()
}