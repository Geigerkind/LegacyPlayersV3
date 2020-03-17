use crate::modules::data::{tools::RetrievePowerType, Data};

#[test]
fn get_power_type() {
    let data = Data::default().init(Some(10));
    let power_type = data.get_power_type(1);
    assert!(power_type.is_some());
    assert_eq!(power_type.unwrap().id, 1);
    let no_power_type = data.get_power_type(0);
    assert!(no_power_type.is_none());
}

#[test]
fn get_all_power_types() {
    let data = Data::default().init(Some(10));
    let power_types = data.get_all_power_types();
    assert!(power_types.len() > 0);
}
