use rocket::State;
use rocket_contrib::json::Json;

use crate::dto::Failure;
use crate::modules::account::guard::ServerOwner;
use crate::modules::armory::Armory;
use crate::modules::armory::dto::CharacterHistoryDto;
use crate::modules::armory::tools::SetCharacterHistory;

#[openapi]
#[post("/character_history", format = "application/json", data = "<character_history>")]
pub fn set_character_history(me: State<Armory>, owner: ServerOwner, character_history: Json<CharacterHistoryDto>) -> Result<(), Failure>
{
  me.set_character_history(owner.0, character_history.into_inner()).and_then(|_| Ok(()))
}