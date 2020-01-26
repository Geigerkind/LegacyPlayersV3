use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::guard::{Expansion, Language};
use crate::modules::tooltip::dto::TooltipFailure;
use crate::modules::tooltip::material::SpellTooltip;
use crate::modules::tooltip::tools::RetrieveSpellTooltip;
use crate::modules::tooltip::Tooltip;

#[openapi]
#[get("/spell/<id>")]
pub fn get_spell(me: State<Tooltip>, data: State<Data>, language: Language, expansion: Expansion, id: u32) -> Result<Json<SpellTooltip>, TooltipFailure>
{
  me.get_spell(&data, language.0, expansion.0, id).and_then(|tooltip| Ok(Json(tooltip)))
}
