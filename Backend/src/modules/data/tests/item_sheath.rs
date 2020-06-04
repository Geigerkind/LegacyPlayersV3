use crate::modules::data::{tools::RetrieveItemSheath, Data};
use crate::start_test_db;

#[test]
fn get_item_sheath() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(26));
    let item_sheath = data.get_item_sheath(1);
    assert!(item_sheath.is_some());
    assert_eq!(item_sheath.unwrap().id, 1);
    let no_item_sheath = data.get_item_sheath(0);
    assert!(no_item_sheath.is_none());
}

#[test]
fn get_all_item_sheaths() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(26));
    let item_sheaths = data.get_all_item_sheaths();
    assert!(item_sheaths.len() > 0);
}
