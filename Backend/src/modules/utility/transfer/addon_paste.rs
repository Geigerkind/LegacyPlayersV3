use crate::modules::utility::domain_value::Paste;
use crate::modules::utility::dto::UtilityFailure;
use crate::modules::utility::Utility;
use rocket::State;
use rocket_contrib::json::Json;
use crate::modules::utility::tools::RetrieveAddonPaste;

#[openapi]
#[get("/addon_paste/<id>")]
pub fn get_addon_paste(me: State<Utility>, id: u32) -> Result<Json<Paste>, UtilityFailure> {
    me.get_addon_paste(id).ok_or(UtilityFailure::InvalidInput).map(Json)
}

#[openapi]
#[get("/addon_paste")]
pub fn get_addon_pastes(me: State<Utility>) -> Json<Vec<Paste>> {
    Json(me.get_addon_pastes())
}