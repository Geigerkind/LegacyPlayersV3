use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::tools::RetrieveSpell;
use crate::modules::data::domain_value::Spell;

#[openapi]
#[get("/spell/<expansion_id>/<spell_id>")]
pub fn get_spell(me: State<Data>, expansion_id: u8, spell_id: u32) -> Option<Json<Spell>> {
  me.get_spell(expansion_id, spell_id)
    .and_then(|result| Some(Json(result)))
}