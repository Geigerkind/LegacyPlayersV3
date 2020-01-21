use rocket::State;
use rocket_contrib::json::Json;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::data::Data;
use crate::modules::data::guard::{Expansion, Language};
use crate::modules::tooltip::tools::RetrieveItemTooltip;
use crate::modules::tooltip::Tooltip;
use crate::modules::tooltip::material::ItemTooltip;

#[openapi]
#[get("/item/<id>")]
pub fn get_item(me: State<Tooltip>, data: State<Data>, language: Language, expansion: Expansion, id: u32) -> Result<Json<ItemTooltip>, Failure>
{
  me.get_item(&data, language.0, expansion.0, id).and_then(|tooltip| Ok(Json(tooltip)))
}

#[openapi]
#[get("/item/<character_gear_id>/<item_id>")]
pub fn get_character_item(me: State<Tooltip>, data: State<Data>, armory: State<Armory>, language: Language, expansion: Expansion, character_gear_id: u32, item_id: u32) -> Result<Json<ItemTooltip>, Failure>
{
  me.get_character_item(&data, &armory, language.0, expansion.0, item_id, character_gear_id).and_then(|tooltip| Ok(Json(tooltip)))
}
