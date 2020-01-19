use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::domain_value::ItemsetName;
use crate::modules::data::tools::RetrieveItemsetName;

#[openapi]
#[get("/itemset_name/<expansion_id>/<itemset_id>")]
pub fn get_itemset_name(me: State<Data>, expansion_id: u8, itemset_id: u16) -> Option<Json<ItemsetName>> {
  me.get_itemset_name(expansion_id, itemset_id)
    .and_then(|result| Some(Json(result)))
}

#[openapi]
#[get("/itemset_ids/<expansion_id>/<itemset_id>")]
pub fn get_itemset_item_ids(me: State<Data>, expansion_id: u8, itemset_id: u16) -> Option<Json<Vec<u32>>> {
  me.get_itemset_item_ids(expansion_id, itemset_id)
    .and_then(|result| Some(Json(result)))
}