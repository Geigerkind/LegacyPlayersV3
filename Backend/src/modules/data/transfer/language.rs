use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::domain_value::Language;
use crate::modules::data::tools::RetrieveLanguage;

#[openapi]
#[get("/language/<id>")]
pub fn get_language(me: State<Data>, id: u8) -> Option<Json<Language>>
{
  me.get_language(id)
    .and_then(|expansion| Some(Json(expansion)))
}

#[openapi]
#[get("/language")]
pub fn get_all_languages(me: State<Data>) -> Json<Vec<Language>>
{
  Json(me.get_all_languages())
}