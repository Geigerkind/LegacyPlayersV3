use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::{
    data::{
        guard::{Expansion, Language},
        Data,
    },
    tooltip::{dto::TooltipFailure, material::SpellTooltip, tools::RetrieveSpellTooltip, Tooltip},
};

#[openapi]
#[get("/spell/<id>")]
pub fn get_spell(me: State<Tooltip>, data: State<Data>, language: Language, expansion: Expansion, id: u32) -> Result<Json<SpellTooltip>, TooltipFailure> {
    me.get_spell(&data, language.0, expansion.0, id).map(Json)
}
