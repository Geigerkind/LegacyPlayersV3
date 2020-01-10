use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::domain_value::ItemDamageType;
use crate::modules::data::tools::RetrieveItemDamageType;

#[openapi]
#[get("/item_damage_type/<id>")]
pub fn get_item_damage_type(me: State<Data>, id: u8) -> Option<Json<ItemDamageType>>
{
  me.get_item_damage_type(id)
    .and_then(|item_damage_type| Some(Json(item_damage_type)))
}

#[openapi]
#[get("/item_damage_type")]
pub fn get_all_item_damage_types(me: State<Data>) -> Json<Vec<ItemDamageType>>
{
  Json(me.get_all_item_damage_types())
}