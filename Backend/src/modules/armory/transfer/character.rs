use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::{
    account::guard::ServerOwner,
    armory::{
        dto::{ArmoryFailure, CharacterDto},
        material::Character,
        tools::{DeleteCharacter, GetCharacter, SetCharacter},
        Armory,
    },
};
use crate::MainDb;

#[openapi(skip)]
#[post("/character", format = "application/json", data = "<character>")]
pub fn set_character(mut db_main: MainDb, me: State<Armory>, owner: ServerOwner, character: Json<CharacterDto>) -> Result<(), ArmoryFailure> {
    me.set_character(&mut *db_main, owner.0, character.into_inner()).map(|_| ())
}

#[openapi]
#[get("/character/<id>")]
pub fn get_character(me: State<Armory>, id: u32) -> Result<Json<Character>, ArmoryFailure> {
    me.get_character(id).map(Json).ok_or(ArmoryFailure::InvalidInput)
}

#[openapi]
#[get("/character/by_name/<name>")]
pub fn get_characters_by_name(me: State<Armory>, name: String) -> Json<Vec<Character>> {
    Json(me.get_characters_by_name(name))
}

#[openapi]
#[get("/character/by_uid/<uid>")]
pub fn get_character_by_uid(me: State<Armory>, owner: ServerOwner, uid: u64) -> Result<Json<Character>, ArmoryFailure> {
    me.get_character_by_uid(owner.0, uid).map(Json).ok_or(ArmoryFailure::InvalidInput)
}

#[openapi(skip)]
#[delete("/character/<id>")]
pub fn delete_character(mut db_main: MainDb, me: State<Armory>, _owner: ServerOwner, id: u32) -> Result<(), ArmoryFailure> {
    me.delete_character(&mut *db_main, id)
}

#[openapi(skip)]
#[delete("/character/by_uid/<uid>")]
pub fn delete_character_by_uid(mut db_main: MainDb, me: State<Armory>, owner: ServerOwner, uid: u64) -> Result<(), ArmoryFailure> {
    me.delete_character_by_uid(&mut *db_main, owner.0, uid)
}
