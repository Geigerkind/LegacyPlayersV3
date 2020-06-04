use crate::modules::data::{tools::RetrieveIcon, Data};
use crate::start_test_db;

#[test]
fn get_icon() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(14));
    let icon = data.get_icon(1);
    assert!(icon.is_some());
    assert_eq!(icon.unwrap().id, 1);
    let no_icon = data.get_icon(0);
    assert!(no_icon.is_none());
}
