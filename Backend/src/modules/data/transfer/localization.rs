use rocket_contrib::json::Json;
use crate::modules::data::Data;
use rocket::State;
use crate::modules::data::tools::RetrieveLocalization;

#[openapi]
#[get("/localization/<language_id>/<localization_id>")]
pub fn get_localization(me: State<Data>, language_id: u8, localization_id: u32) -> Option<Json<String>> {
  me.get_localization(language_id, localization_id)
    .and_then(|result| Some(Json(result)))
}