use crate::modules::data::{tools::RetrieveItemQuality, Data};

#[test]
fn get_item_quality() {
    let data = Data::default().init(Some(24));
    let item_quality = data.get_item_quality(1);
    assert!(item_quality.is_some());
    assert_eq!(item_quality.unwrap().id, 1);
    let no_item_quality = data.get_item_quality(0);
    assert!(no_item_quality.is_none());
}

#[test]
fn get_all_item_qualities() {
    let data = Data::default().init(Some(24));
    let item_qualities = data.get_all_item_qualities();
    assert!(item_qualities.len() > 0);
}
