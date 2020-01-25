use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::armory::Armory;
use crate::modules::armory::dto::{CharacterSearchFilter, CharacterSearchResult};
use crate::modules::armory::tools::PerformCharacterSearch;
use crate::modules::data::Data;
use crate::modules::data::guard::Language;

#[openapi]
#[post("/character_search", format = "application/json", data = "<filter>")]
pub fn get_character_search_result(me: State<Armory>, data: State<Data>, language: Language, filter: Json<CharacterSearchFilter>) -> Json<Vec<CharacterSearchResult>>
{
  Json(me.get_character_search_result(&data, language.0, filter.into_inner()))
}