use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::domain_value::Enchant;
use crate::modules::data::tools::RetrieveEnchant;
use crate::modules::data::Data;

#[openapi]
#[get("/enchant/<expansion_id>/<enchant_id>")]
pub fn get_enchant(me: State<Data>, expansion_id: u8, enchant_id: u32) -> Option<Json<Enchant>> {
    me.get_enchant(expansion_id, enchant_id)
        .and_then(|result| Some(Json(result)))
}
