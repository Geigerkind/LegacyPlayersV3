use rocket::State;
use rocket_contrib::json::Json;
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::modules::account::guard::ServerOwner;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::tools::ProcessMessages;

#[openapi]
#[post("/package", format = "application/json", data = "<params>")]
pub fn get_package(me: State<LiveDataProcessor>, owner: ServerOwner, params: Json<Vec<String>>) -> Result<(), LiveDataProcessorFailure> {
  me.process_messages(owner.0, params.into_inner())
}