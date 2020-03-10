use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::domain_value::DispelType;
use crate::modules::data::tools::RetrieveDispelType;
use crate::modules::data::Data;

#[openapi]
#[get("/dispel_type/<id>")]
pub fn get_dispel_type(me: State<Data>, id: u8) -> Option<Json<DispelType>> {
    me.get_dispel_type(id)
        .and_then(|dispel_type| Some(Json(dispel_type)))
}

#[openapi]
#[get("/dispel_type")]
pub fn get_all_dispel_types(me: State<Data>) -> Json<Vec<DispelType>> {
    Json(me.get_all_dispel_types())
}
