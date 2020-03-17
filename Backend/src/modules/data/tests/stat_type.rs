use crate::modules::data::{tools::RetrieveStatType, Data};

#[test]
fn get_stat_type() {
    let data = Data::default().init(Some(11));
    let stat_type = data.get_stat_type(1);
    assert!(stat_type.is_some());
    assert_eq!(stat_type.unwrap().id, 1);
    let no_stat_type = data.get_stat_type(0);
    assert!(no_stat_type.is_none());
}

#[test]
fn get_all_stat_types() {
    let data = Data::default().init(Some(11));
    let stat_types = data.get_all_stat_types();
    assert!(stat_types.len() > 0);
}
