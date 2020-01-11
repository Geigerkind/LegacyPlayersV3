use rocket::State;
use rocket_contrib::json::Json;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::material::Character;
use crate::modules::account::guard::ServerOwner;
use crate::modules::armory::tools::SetCharacter;

#[openapi]
#[post("/set", format = "application/json", data = "<character>")]
pub fn set_character(me: State<Armory>, owner: ServerOwner, character: Json<Character>) -> Result<(), Failure>
{
  me.set_character(owner.0, character.into_inner()).and_then(|_| Ok(()))
}