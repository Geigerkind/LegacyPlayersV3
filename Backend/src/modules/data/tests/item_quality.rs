use crate::modules::data::{tools::RetrieveItemQuality, Data};
use crate::start_test_db;

#[test]
fn get_item_quality() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(24));
    let item_quality = data.get_item_quality(1);
    assert!(item_quality.is_some());
    assert_eq!(item_quality.unwrap().id, 1);
    let no_item_quality = data.get_item_quality(0);
    assert!(no_item_quality.is_none());
}

#[test]
fn get_all_item_qualities() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(24));
    let item_qualities = data.get_all_item_qualities();
    assert!(item_qualities.len() > 0);
}
