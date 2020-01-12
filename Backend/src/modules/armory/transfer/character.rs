use rocket::State;
use rocket_contrib::json::Json;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::account::guard::ServerOwner;
use crate::modules::armory::tools::{SetCharacter, GetCharacter, DeleteCharacter};
use crate::modules::armory::dto::CharacterDto;
use crate::modules::armory::material::Character;

#[openapi]
#[post("/character", format = "application/json", data = "<character>")]
pub fn set_character(me: State<Armory>, owner: ServerOwner, character: Json<CharacterDto>) -> Result<(), Failure>
{
  me.set_character(owner.0, character.into_inner()).and_then(|_| Ok(()))
}

#[openapi]
#[get("/character/<id>")]
pub fn get_character(me: State<Armory>, id: u32) -> Result<Json<Character>, Failure>
{
  me.get_character(id).and_then(|character| Some(Json(character))).ok_or(Failure::InvalidInput)
}

#[openapi]
#[get("/character/uid/<uid>")]
pub fn get_character_by_uid(me: State<Armory>, owner: ServerOwner, uid: u64) -> Result<Json<Character>, Failure>
{
  me.get_character_by_uid(owner.0, uid).and_then(|character| Some(Json(character))).ok_or(Failure::InvalidInput)
}

#[openapi]
#[delete("/character/<id>")]
pub fn delete_character(me: State<Armory>, _owner: ServerOwner, id: u32) -> Result<(), Failure>
{
  me.delete_character(id)
}

#[openapi]
#[delete("/character/uid/<uid>")]
pub fn delete_character_by_uid(me: State<Armory>, owner: ServerOwner, uid: u64) -> Result<(), Failure>
{
  me.delete_character_by_uid(owner.0, uid)
}

