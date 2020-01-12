use rocket::State;
use rocket_contrib::json::Json;

use crate::dto::Failure;
use crate::modules::account::guard::ServerOwner;
use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::Guild;
use crate::modules::armory::tools::{GetGuild, DeleteGuild, CreateGuild, UpdateGuild};

#[openapi]
#[get("/guild/<id>")]
pub fn get_guild(me: State<Armory>, id: u32) -> Result<Json<Guild>, Failure>
{
  me.get_guild(id).and_then(|guild| Some(Json(guild))).ok_or(Failure::InvalidInput)
}

#[openapi]
#[get("/guild/<server_id>/<guild_name>")]
pub fn get_guild_by_name(me: State<Armory>, server_id: u32, guild_name: String) -> Result<Json<Guild>, Failure>
{
  me.get_guild_by_name(server_id, guild_name).and_then(|guild| Some(Json(guild))).ok_or(Failure::InvalidInput)
}

#[openapi]
#[post("/guild", format = "application/json", data = "<guild_name>")]
pub fn create_guild(me: State<Armory>, owner: ServerOwner, guild_name: Json<String>) -> Result<(), Failure>
{
  me.create_guild(owner.0, guild_name.into_inner()).and_then(|_| Ok(()))
}

#[openapi]
#[post("/guild/<guild_id>", format = "application/json", data = "<guild_name>")]
pub fn update_guild_name(me: State<Armory>, _owner: ServerOwner, guild_id: u32, guild_name: Json<String>) -> Result<(), Failure>
{
  me.update_guild_name(guild_id, guild_name.into_inner()).and_then(|_| Ok(()))
}

#[openapi]
#[delete("/guild/<id>")]
pub fn delete_guild(me: State<Armory>, _owner: ServerOwner, id: u32) -> Result<(), Failure>
{
  me.delete_guild(id)
}

#[openapi]
#[delete("/guild/name/<name>")]
pub fn delete_guild_by_name(me: State<Armory>, owner: ServerOwner, name: String) -> Result<(), Failure>
{
  me.delete_guild_by_name(owner.0, name)
}