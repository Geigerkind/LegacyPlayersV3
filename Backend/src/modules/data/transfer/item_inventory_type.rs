use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::domain_value::ItemInventoryType;
use crate::modules::data::tools::RetrieveItemInventoryType;
use crate::modules::data::Data;

#[openapi]
#[get("/item_inventory_type/<id>")]
pub fn get_item_inventory_type(me: State<Data>, id: u8) -> Option<Json<ItemInventoryType>> {
    me.get_item_inventory_type(id)
        .and_then(|item_inventory_type| Some(Json(item_inventory_type)))
}

#[openapi]
#[get("/item_inventory_type")]
pub fn get_all_item_inventory_types(me: State<Data>) -> Json<Vec<ItemInventoryType>> {
    Json(me.get_all_item_inventory_types())
}
