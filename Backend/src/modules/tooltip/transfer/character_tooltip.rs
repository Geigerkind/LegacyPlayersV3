use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::{
    armory::Armory,
    data::{guard::Language, Data},
    tooltip::{dto::TooltipFailure, material::CharacterTooltip, tools::RetrieveCharacterTooltip, Tooltip},
};

#[openapi]
#[get("/character/<id>")]
pub fn get_character(me: State<Tooltip>, data: State<Data>, armory: State<Armory>, language: Language, id: u32) -> Result<Json<CharacterTooltip>, TooltipFailure> {
    me.get_character(&data, &armory, language.0, id).map(Json)
}
