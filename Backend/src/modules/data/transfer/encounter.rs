use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::{domain_value::Encounter, tools::RetrieveEncounter, Data};

#[openapi]
#[get("/encounter/<id>")]
pub fn get_encounter(me: State<Data>, id: u32) -> Option<Json<Encounter>> {
    me.get_encounter(id).map(Json)
}

#[openapi]
#[get("/encounter")]
pub fn get_all_encounters(me: State<Data>) -> Json<Vec<Encounter>> {
    Json(me.get_all_encounters())
}
