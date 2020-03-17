use crate::modules::data::{tools::RetrieveItemRandomPropertyPoints, Data};

#[test]
fn get_item_random_property_points() {
    let data = Data::default().init(Some(32));
    let item_random_property = data.get_item_random_property_points(2, 5);
    assert!(item_random_property.is_some());
    let unpacked_item_random_property = item_random_property.unwrap();
    assert_eq!(unpacked_item_random_property.item_level, 5);
    assert_eq!(unpacked_item_random_property.expansion_id, 2);
    let no_item_random_property = data.get_item_random_property_points(0, 0);
    assert!(no_item_random_property.is_none());
}
