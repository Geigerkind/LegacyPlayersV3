use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::armory::Armory;
use crate::modules::data::Data;
use crate::modules::tooltip::dto::TooltipFailure;
use crate::modules::tooltip::material::CharacterTooltip;
use crate::modules::tooltip::tools::RetrieveCharacterTooltip;
use crate::modules::tooltip::Tooltip;
use crate::modules::data::guard::Language;

#[openapi]
#[get("/character/<id>")]
pub fn get_character(me: State<Tooltip>, data: State<Data>, armory: State<Armory>, language: Language, id: u32) -> Result<Json<CharacterTooltip>, TooltipFailure>
{
  me.get_character(&data, &armory, language.0, id).and_then(|tooltip| Ok(Json(tooltip)))
}
