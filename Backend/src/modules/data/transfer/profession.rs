use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::domain_value::Profession;
use crate::modules::data::tools::RetrieveProfession;
use crate::modules::data::Data;

#[openapi]
#[get("/profession/<id>")]
pub fn get_profession(me: State<Data>, id: u16) -> Option<Json<Profession>> {
    me.get_profession(id)
        .and_then(|profession| Some(Json(profession)))
}

#[openapi]
#[get("/profession")]
pub fn get_all_professions(me: State<Data>) -> Json<Vec<Profession>> {
    Json(me.get_all_professions())
}
