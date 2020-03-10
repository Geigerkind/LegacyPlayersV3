use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::domain_value::ItemClass;
use crate::modules::data::tools::RetrieveItemClass;
use crate::modules::data::Data;

#[openapi]
#[get("/item_class/<id>")]
pub fn get_item_class(me: State<Data>, id: u8) -> Option<Json<ItemClass>> {
    me.get_item_class(id)
        .and_then(|item_class| Some(Json(item_class)))
}

#[openapi]
#[get("/item_class")]
pub fn get_all_item_classes(me: State<Data>) -> Json<Vec<ItemClass>> {
    Json(me.get_all_item_classes())
}
