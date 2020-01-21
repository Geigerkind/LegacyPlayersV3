use rocket::State;
use rocket_contrib::json::Json;

use crate::dto::Failure;
use crate::modules::data::Data;
use crate::modules::data::guard::{Expansion, Language};
use crate::modules::tooltip::tools::RetrieveSpellTooltip;
use crate::modules::tooltip::Tooltip;
use crate::modules::tooltip::material::SpellTooltip;

#[openapi]
#[get("/spell/<id>")]
pub fn get_spell(me: State<Tooltip>, data: State<Data>, language: Language, expansion: Expansion, id: u32) -> Result<Json<SpellTooltip>, Failure>
{
  me.get_spell(&data, language.0, expansion.0, id).and_then(|tooltip| Ok(Json(tooltip)))
}
