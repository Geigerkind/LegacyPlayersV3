use crate::modules::account::guard::ServerOwner;
use crate::MainDb;
use rocket::State;
use rocket_contrib::json::Json;
use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, InstanceResetDto};
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::modules::live_data_processor::tools::server::HandleInstanceReset;

#[openapi]
#[post("/instance_reset", format = "application/json", data = "<instance_resets>")]
pub fn set_instance_resets(mut db_main: MainDb, me: State<LiveDataProcessor>, owner: ServerOwner, instance_resets: Json<Vec<InstanceResetDto>>) -> Result<(), LiveDataProcessorFailure> {
    if let Some(server) = me.servers.get(&owner.0) {
        let mut server = server.write().unwrap();
        return server.set_instance_resets(&mut *db_main, instance_resets.into_inner());
    }
    Err(LiveDataProcessorFailure::InvalidInput)
}
