use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::armory::Armory;
use crate::modules::armory::dto::{CharacterSearchFilter, CharacterSearchResult};
use crate::modules::armory::tools::PerformCharacterSearch;
use crate::modules::data::Data;

#[openapi]
#[post("/character_search", format = "application/json", data = "<filter>")]
pub fn get_character_search_result(me: State<Armory>, data: State<Data>, filter: Json<CharacterSearchFilter>) -> Json<Vec<CharacterSearchResult>>
{
  Json(me.get_character_search_result(&data, filter.into_inner()))
}