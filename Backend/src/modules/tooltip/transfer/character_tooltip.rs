use rocket::State;
use rocket_contrib::json::Json;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::tooltip::material::CharacterTooltip;
use crate::modules::tooltip::tools::RetrieveCharacterTooltip;
use crate::modules::tooltip::Tooltip;

#[openapi]
#[get("/character/<id>")]
pub fn get_character(me: State<Tooltip>, armory: State<Armory>, id: u32) -> Result<Json<CharacterTooltip>, Failure>
{
    me.get_character(&armory, id).and_then(|tooltip| Ok(Json(tooltip)))
}
