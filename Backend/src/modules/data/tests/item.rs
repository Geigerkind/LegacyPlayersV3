use crate::modules::data::{tools::RetrieveItem, Data};

#[test]
fn get_item() {
    let data = Data::default().init(Some(15));
    let item = data.get_item(1, 25);
    assert!(item.is_some());
    let unpacked_item = item.unwrap();
    assert_eq!(unpacked_item.id, 25);
    assert_eq!(unpacked_item.expansion_id, 1);
    let no_item = data.get_item(0, 0);
    assert!(no_item.is_none());
}
