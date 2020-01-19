use rocket::State;
use rocket_contrib::json::Json;

use crate::dto::Failure;
use crate::modules::tooltip::domain_value::ItemTooltip;
use crate::modules::tooltip::tools::RetrieveItemTooltip;
use crate::modules::tooltip::Tooltip;
use crate::modules::data::Data;
use crate::modules::data::guard::Language;

#[openapi]
#[get("/item/<id>")]
pub fn get_item(me: State<Tooltip>, data: State<Data>, language: Language, id: u32) -> Result<Json<ItemTooltip>, Failure>
{
  me.get_item(&data, language.0, id).and_then(|tooltip| Ok(Json(tooltip)))
}

#[openapi]
#[get("/item/<character_history_id>/<item_id>")]
pub fn get_character_item(me: State<Tooltip>, data: State<Data>, language: Language, character_history_id: u32, item_id: u32) -> Result<Json<ItemTooltip>, Failure>
{
  me.get_character_item(&data, language.0, item_id, character_history_id).and_then(|tooltip| Ok(Json(tooltip)))
}
