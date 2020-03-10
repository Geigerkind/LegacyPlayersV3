use crate::modules::data::tools::RetrieveItemDamageType;
use crate::modules::data::Data;

#[test]
fn get_item_damage_type() {
    let data = Data::default().init(Some(21));
    let item_damage_type = data.get_item_damage_type(1);
    assert!(item_damage_type.is_some());
    assert_eq!(item_damage_type.unwrap().id, 1);
    let no_item_damage_type = data.get_item_damage_type(0);
    assert!(no_item_damage_type.is_none());
}

#[test]
fn get_all_item_damage_types() {
    let data = Data::default().init(Some(21));
    let item_damage_types = data.get_all_item_damage_types();
    assert!(item_damage_types.len() > 0);
}
