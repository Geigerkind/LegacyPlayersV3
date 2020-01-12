use rocket::State;
use rocket_contrib::json::Json;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::Guild;
use crate::modules::armory::tools::GetGuild;

#[openapi]
#[get("/guild/<id>")]
pub fn get_guild(me: State<Armory>, id: u32) -> Result<Json<Guild>, Failure>
{
  me.get_guild(id).and_then(|guild| Some(Json(guild))).ok_or(Failure::InvalidInput)
}