use crate::modules::live_data_processor::LiveDataProcessor;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::domain_value::Event;

use rocket::State;
use rocket_contrib::json::Json;

#[openapi]
#[get("/instance/<server_id>/<instance_id>")]
pub fn get_instance(me: State<LiveDataProcessor>, server_id: u32, instance_id: u32) -> Result<Json<Vec<Event>>, LiveDataProcessorFailure> {
    me.servers.get(&server_id).unwrap().read().unwrap()
        .committed_events.get(&instance_id)
        .ok_or(LiveDataProcessorFailure::InvalidInput)
        .map(|vec| vec.clone())
        .map(Json)
}