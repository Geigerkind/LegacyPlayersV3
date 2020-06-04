use crate::modules::data::{tools::RetrieveItemBonding, Data};
use crate::start_test_db;

#[test]
fn get_item_bonding() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(18));
    let item_bonding = data.get_item_bonding(1);
    assert!(item_bonding.is_some());
    assert_eq!(item_bonding.unwrap().id, 1);
    let no_item_bonding = data.get_item_bonding(0);
    assert!(no_item_bonding.is_none());
}

#[test]
fn get_all_item_bondings() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(18));
    let item_bondings = data.get_all_item_bondings();
    assert!(item_bondings.len() > 0);
}
