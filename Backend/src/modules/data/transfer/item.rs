use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::domain_value::Item;
use crate::modules::data::tools::RetrieveItem;
use crate::modules::data::Data;

#[openapi]
#[get("/item/<expansion_id>/<item_id>")]
pub fn get_item(me: State<Data>, expansion_id: u8, item_id: u32) -> Option<Json<Item>> {
    me.get_item(expansion_id, item_id)
        .map(Json)
}
