use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::domain_value::Expansion;
use crate::modules::data::tools::RetrieveExpansion;

#[openapi]
#[get("/expansion/<id>")]
pub fn get_expansion(me: State<Data>, id: u8) -> Option<Json<Expansion>>
{
  me.get_expansion(id)
    .and_then(|expansion| Some(Json(expansion)))
}

#[openapi]
#[get("/expansion")]
pub fn get_all_expansions(me: State<Data>) -> Json<Vec<Expansion>>
{
  Json(me.get_all_expansions())
}