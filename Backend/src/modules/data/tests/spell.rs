use crate::modules::data::{tools::RetrieveSpell, Data};
use crate::start_test_db;

#[test]
fn get_spell() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(8));
    let spell = data.get_spell(1, 1);
    assert!(spell.is_some());
    let unpacked_spell = spell.unwrap();
    assert_eq!(unpacked_spell.id, 1);
    assert_eq!(unpacked_spell.expansion_id, 1);
    let no_spell = data.get_spell(0, 0);
    assert!(no_spell.is_none());
}
