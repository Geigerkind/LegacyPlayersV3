use crate::modules::data::{tools::RetrieveItem, Data};
use crate::start_test_db;

#[test]
fn get_item() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(15));
    let item = data.get_item(1, 25);
    assert!(item.is_some());
    let unpacked_item = item.unwrap();
    assert_eq!(unpacked_item.id, 25);
    assert_eq!(unpacked_item.expansion_id, 1);
    let no_item = data.get_item(0, 0);
    assert!(no_item.is_none());
}
