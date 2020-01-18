use rocket::State;
use rocket_contrib::json::Json;

use crate::dto::Failure;
use crate::modules::account::guard::ServerOwner;
use crate::modules::armory::Armory;
use crate::modules::armory::dto::GuildDto;
use crate::modules::armory::material::Guild;
use crate::modules::armory::tools::{CreateGuild, DeleteGuild, GetGuild, UpdateGuild};

#[openapi]
#[get("/guild/<id>")]
pub fn get_guild(me: State<Armory>, id: u32) -> Result<Json<Guild>, Failure>
{
  me.get_guild(id).and_then(|guild| Some(Json(guild))).ok_or(Failure::InvalidInput)
}

#[openapi]
#[get("/guild/by_name/<guild_name>")]
pub fn get_guilds_by_name(me: State<Armory>, guild_name: String) -> Json<Vec<Guild>>
{
  Json(me.get_guilds_by_name(guild_name))
}

#[openapi]
#[post("/guild", format = "application/json", data = "<guild>")]
pub fn create_guild(me: State<Armory>, owner: ServerOwner, guild: Json<GuildDto>) -> Result<(), Failure>
{
  me.create_guild(owner.0, guild.into_inner()).and_then(|_| Ok(()))
}

#[openapi]
#[post("/guild/<uid>", format = "application/json", data = "<guild_name>")]
pub fn update_guild_name(me: State<Armory>, owner: ServerOwner, uid: u64, guild_name: Json<String>) -> Result<(), Failure>
{
  me.update_guild_name(owner.0, uid, guild_name.into_inner()).and_then(|_| Ok(()))
}

#[openapi]
#[delete("/guild/<id>")]
pub fn delete_guild(me: State<Armory>, _owner: ServerOwner, id: u32) -> Result<(), Failure>
{
  me.delete_guild(id)
}

#[openapi]
#[delete("/guild/by_uid/<uid>")]
pub fn delete_guild_by_uid(me: State<Armory>, owner: ServerOwner, uid: u64) -> Result<(), Failure>
{
  me.delete_guild_by_uid(owner.0, uid)
}