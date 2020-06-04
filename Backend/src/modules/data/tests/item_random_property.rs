use crate::modules::data::{tools::RetrieveItemRandomProperty, Data};
use crate::start_test_db;

#[test]
fn get_item_random_property() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(25));
    let item_random_property = data.get_item_random_property(1, 5);
    assert!(item_random_property.is_some());
    let unpacked_item_random_property = item_random_property.unwrap();
    assert_eq!(unpacked_item_random_property.id, 5);
    assert_eq!(unpacked_item_random_property.expansion_id, 1);
    let no_item_random_property = data.get_item_random_property(0, 0);
    assert!(no_item_random_property.is_none());
}
