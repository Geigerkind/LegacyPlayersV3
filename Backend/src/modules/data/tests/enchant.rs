use crate::modules::data::{tools::RetrieveEnchant, Data};
use crate::start_test_db;

#[test]
fn get_enchant() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(17));
    let enchant = data.get_enchant(1, 1);
    assert!(enchant.is_some());
    let unpacked_enchant = enchant.unwrap();
    assert_eq!(unpacked_enchant.id, 1);
    assert_eq!(unpacked_enchant.expansion_id, 1);
    let no_enchant = data.get_enchant(0, 0);
    assert!(no_enchant.is_none());
}
