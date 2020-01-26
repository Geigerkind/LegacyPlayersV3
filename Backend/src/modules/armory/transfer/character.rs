use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::account::guard::ServerOwner;
use crate::modules::armory::Armory;
use crate::modules::armory::dto::{ArmoryFailure, CharacterDto};
use crate::modules::armory::material::Character;
use crate::modules::armory::tools::{DeleteCharacter, GetCharacter, SetCharacter};

#[openapi]
#[post("/character", format = "application/json", data = "<character>")]
pub fn set_character(me: State<Armory>, owner: ServerOwner, character: Json<CharacterDto>) -> Result<(), ArmoryFailure>
{
  me.set_character(owner.0, character.into_inner()).and_then(|_| Ok(()))
}

#[openapi]
#[get("/character/<id>")]
pub fn get_character(me: State<Armory>, id: u32) -> Result<Json<Character>, ArmoryFailure>
{
  me.get_character(id).and_then(|character| Some(Json(character))).ok_or(ArmoryFailure::InvalidInput)
}

#[openapi]
#[get("/character/by_name/<name>")]
pub fn get_characters_by_name(me: State<Armory>, name: String) -> Json<Vec<Character>>
{
  Json(me.get_characters_by_name(name))
}

#[openapi]
#[get("/character/by_uid/<uid>")]
pub fn get_character_by_uid(me: State<Armory>, owner: ServerOwner, uid: u64) -> Result<Json<Character>, ArmoryFailure>
{
  me.get_character_by_uid(owner.0, uid).and_then(|character| Some(Json(character))).ok_or(ArmoryFailure::InvalidInput)
}

#[openapi]
#[delete("/character/<id>")]
pub fn delete_character(me: State<Armory>, _owner: ServerOwner, id: u32) -> Result<(), ArmoryFailure>
{
  me.delete_character(id)
}

#[openapi]
#[delete("/character/by_uid/<uid>")]
pub fn delete_character_by_uid(me: State<Armory>, owner: ServerOwner, uid: u64) -> Result<(), ArmoryFailure>
{
  me.delete_character_by_uid(owner.0, uid)
}

