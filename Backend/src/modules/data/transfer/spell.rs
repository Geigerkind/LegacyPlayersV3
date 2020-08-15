use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::domain_value::Localized;
use crate::modules::data::dto::BasicSpell;
use crate::modules::data::guard::Language;
use crate::modules::data::tools::{RetrieveIcon, RetrieveLocalization};
use crate::modules::data::{domain_value::Spell, tools::RetrieveSpell, Data};

#[openapi]
#[get("/spell/<expansion_id>/<spell_id>")]
pub fn get_spell(me: State<Data>, expansion_id: u8, spell_id: u32) -> Option<Json<Spell>> {
    me.get_spell(expansion_id, spell_id).map(Json)
}

#[openapi]
#[get("/spell/localized/basic_spell/<expansion_id>/<spell_id>")]
pub fn get_localized_basic_spell(me: State<Data>, language: Language, expansion_id: u8, spell_id: u32) -> Option<Json<Localized<BasicSpell>>> {
    me.get_spell(expansion_id, spell_id)
        .map(|spell| Localized {
            base: BasicSpell {
                id: spell_id,
                icon: me.get_icon(spell.icon).unwrap().name,
                school: spell.school_mask,
            },
            localization: me.get_localization(language.0, spell.localization_id).map(|localization| localization.content).unwrap_or_else(|| String::from("NOT LOCALIZED")),
        })
        .map(Json)
}
