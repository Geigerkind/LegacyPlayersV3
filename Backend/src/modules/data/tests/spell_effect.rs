use crate::modules::data::{tools::RetrieveSpellEffect, Data};
use crate::start_test_db;

#[test]
fn get_spell_effects() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(12));
    let spell_effects = data.get_spell_effects(1, 1);
    assert!(spell_effects.is_some());
    let spell_effects_vec = spell_effects.unwrap();
    assert!(spell_effects_vec.len() > 0);
    assert_eq!(spell_effects_vec[0].expansion_id, 1);
    assert_eq!(spell_effects_vec[0].spell_id, 1);
    let no_spell_effects = data.get_spell_effects(0, 0);
    assert!(no_spell_effects.is_none());
}
