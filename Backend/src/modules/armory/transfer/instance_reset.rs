use rocket::State;
use rocket_contrib::json::Json;
use crate::modules::armory::Armory;
use crate::modules::account::guard::ServerOwner;
use crate::modules::armory::dto::{ArmoryFailure, InstanceResetDto};

#[openapi]
#[post("/instance_reset", format = "application/json", data = "<instance_resets>")]
pub fn set_instance_resets(me: State<Armory>, owner: ServerOwner, instance_resets: Json<Vec<InstanceResetDto>>) -> Result<(), ArmoryFailure> {
  unimplemented!()
}