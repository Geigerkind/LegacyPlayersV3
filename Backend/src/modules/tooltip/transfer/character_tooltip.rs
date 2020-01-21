use rocket::State;
use rocket_contrib::json::Json;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::data::Data;
use crate::modules::tooltip::material::CharacterTooltip;
use crate::modules::tooltip::tools::RetrieveCharacterTooltip;
use crate::modules::tooltip::Tooltip;

#[openapi]
#[get("/character/<id>")]
pub fn get_character(me: State<Tooltip>, data: State<Data>, armory: State<Armory>, id: u32) -> Result<Json<CharacterTooltip>, Failure>
{
    me.get_character(&data, &armory, id).and_then(|tooltip| Ok(Json(tooltip)))
}
