use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::armory::Armory;
use crate::modules::armory::dto::{CharacterViewerDto, ArmoryFailure};
use crate::modules::armory::tools::{CharacterViewer, GetCharacter};
use crate::modules::data::Data;
use crate::modules::data::guard::Language;
use crate::modules::data::tools::RetrieveServer;

#[openapi]
#[get("/character_viewer/<server_name>/<character_name>")]
pub fn get_character_viewer(me: State<Armory>, data: State<Data>, language: Language, server_name: String, character_name: String) -> Result<Json<CharacterViewerDto>, ArmoryFailure>
{
    data.get_server_by_name(server_name).ok_or(ArmoryFailure::InvalidInput)
        .and_then(|server|
            me.get_character_by_name(server.id, character_name).ok_or(ArmoryFailure::InvalidInput)
                .and_then(|character|
                    me.get_character_viewer(&data, language.0, character.id)
                        .and_then(|result| Ok(Json(result)))))
}

#[openapi]
#[get("/character_viewer/<server_name>/<character_name>/<character_history_id>")]
pub fn get_character_viewer_by_history(me: State<Armory>, data: State<Data>, language: Language, server_name: String, character_name: String, character_history_id: u32) -> Result<Json<CharacterViewerDto>, ArmoryFailure>
{
    data.get_server_by_name(server_name).ok_or(ArmoryFailure::InvalidInput)
        .and_then(|server|
            me.get_character_by_name(server.id, character_name).ok_or(ArmoryFailure::InvalidInput)
                .and_then(|character|
                    me.get_character_viewer_by_history_id(&data, language.0, character_history_id, character.id)
                        .and_then(|result| Ok(Json(result)))))
}