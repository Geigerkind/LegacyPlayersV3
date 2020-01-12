use rocket::State;
use rocket_contrib::json::Json;

use crate::dto::Failure;
use crate::modules::account::guard::ServerOwner;
use crate::modules::armory::Armory;
use crate::modules::armory::dto::CharacterHistoryDto;
use crate::modules::armory::tools::{SetCharacterHistory, GetCharacterHistory, DeleteCharacterHistory};
use crate::modules::armory::material::CharacterHistory;

#[openapi]
#[post("/character_history", format = "application/json", data = "<character_history>")]
pub fn set_character_history(me: State<Armory>, owner: ServerOwner, character_history: Json<CharacterHistoryDto>) -> Result<(), Failure>
{
  me.set_character_history(owner.0, character_history.into_inner()).and_then(|_| Ok(()))
}

#[openapi]
#[get("/character_history/<id>")]
pub fn get_character_history(me: State<Armory>, id: u32) -> Result<Json<CharacterHistory>, Failure>
{
  me.get_character_history(id).and_then(|character_history| Ok(Json(character_history)))
}

#[openapi]
#[delete("/character_history/<id>")]
pub fn delete_character_history(me: State<Armory>, id: u32) -> Result<(), Failure>
{
  me.delete_character_history(id)
}