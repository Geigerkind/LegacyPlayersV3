use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::domain_value::ItemEffect;
use crate::modules::data::tools::RetrieveItemEffect;
use crate::modules::data::Data;

#[openapi]
#[get("/item_effect/<expansion_id>/<item_id>")]
pub fn get_item_effect(
    me: State<Data>,
    expansion_id: u8,
    item_id: u32,
) -> Option<Json<Vec<ItemEffect>>> {
    me.get_item_effect(expansion_id, item_id)
        .and_then(|result| Some(Json(result)))
}
