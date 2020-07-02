use crate::modules::account::guard::ServerOwner;
use crate::modules::armory::dto::{ArmoryFailure, InstanceResetDto};
use crate::modules::armory::tools::HandleInstanceReset;
use crate::modules::armory::Armory;
use crate::MainDb;
use rocket::State;
use rocket_contrib::json::Json;

#[openapi]
#[post("/instance_reset", format = "application/json", data = "<instance_resets>")]
pub fn set_instance_resets(mut db_main: MainDb, me: State<Armory>, owner: ServerOwner, instance_resets: Json<Vec<InstanceResetDto>>) -> Result<(), ArmoryFailure> {
    me.set_instance_resets(&mut *db_main, owner.0, instance_resets.into_inner())
}
