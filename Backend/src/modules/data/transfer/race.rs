use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::domain_value::Race;
use crate::modules::data::tools::RetrieveRace;

#[openapi]
#[get("/race/<id>")]
pub fn get_race(me: State<Data>, id: u8) -> Option<Json<Race>>
{
  me.get_race(id)
    .and_then(|expansion| Some(Json(expansion)))
}

#[openapi]
#[get("/race")]
pub fn get_all_races(me: State<Data>) -> Json<Vec<Race>>
{
  Json(me.get_all_races())
}