use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::{
    armory::{
        dto::{ArmoryFailure, CharacterViewerDto},
        tools::{CharacterViewer, GetCharacter},
        Armory,
    },
    data::{guard::Language, tools::RetrieveServer, Data},
};
use crate::MainDb;

#[openapi]
#[get("/character_viewer/<server_name>/<character_name>")]
pub fn get_character_viewer(mut db_main: MainDb, me: State<Armory>, data: State<Data>, language: Language, server_name: String, character_name: String) -> Result<Json<CharacterViewerDto>, ArmoryFailure> {
    data.get_server_by_name(server_name).ok_or(ArmoryFailure::InvalidInput).and_then(|server| {
        me.get_character_by_name(server.id, character_name)
            .ok_or(ArmoryFailure::InvalidInput)
            .and_then(|character| me.get_character_viewer(&mut *db_main, &data, language.0, character.id).map(Json))
    })
}

#[openapi]
#[get("/character_viewer/<server_name>/<character_name>/<character_history_id>")]
pub fn get_character_viewer_by_history(mut db_main: MainDb, me: State<Armory>, data: State<Data>, language: Language, server_name: String, character_name: String, character_history_id: u32) -> Result<Json<CharacterViewerDto>, ArmoryFailure> {
    data.get_server_by_name(server_name).ok_or(ArmoryFailure::InvalidInput).and_then(|server| {
        me.get_character_by_name(server.id, character_name)
            .ok_or(ArmoryFailure::InvalidInput)
            .and_then(|character| me.get_character_viewer_by_history_id(&mut *db_main, &data, language.0, character_history_id, character.id).map(Json))
    })
}
