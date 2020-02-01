use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::armory::Armory;
use crate::modules::armory::dto::{CharacterViewerDto, ArmoryFailure};
use crate::modules::armory::tools::CharacterViewer;
use crate::modules::data::Data;
use crate::modules::data::guard::Language;

#[openapi]
#[get("/character_viewer/<character_id>")]
pub fn get_character_viewer(me: State<Armory>, data: State<Data>, language: Language, character_id: u32) -> Result<Json<CharacterViewerDto>, ArmoryFailure>
{
  me.get_character_viewer(&data, language.0, character_id).and_then(|result| Ok(Json(result)))
}

#[openapi]
#[get("/character_viewer/<character_id>/<character_history_id>")]
pub fn get_character_viewer_by_history(me: State<Armory>, data: State<Data>, language: Language, character_id: u32, character_history_id: u32) -> Result<Json<CharacterViewerDto>, ArmoryFailure>
{
  me.get_character_viewer_by_history_id(&data, language.0, character_history_id, character_id).and_then(|result| Ok(Json(result)))
}