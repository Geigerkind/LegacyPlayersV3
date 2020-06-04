use crate::modules::data::{tools::RetrieveTitle, Data};
use crate::start_test_db;

#[test]
fn get_title() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(31));
    let title = data.get_title(1);
    assert!(title.is_some());
    assert_eq!(title.unwrap().id, 1);
    let no_title = data.get_title(0);
    assert!(no_title.is_none());
}

#[test]
fn get_all_titles() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(31));
    let titles = data.get_all_titles();
    assert!(titles.len() > 0);
}
