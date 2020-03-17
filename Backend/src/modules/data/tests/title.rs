use crate::modules::data::{tools::RetrieveTitle, Data};

#[test]
fn get_title() {
    let data = Data::default().init(Some(31));
    let title = data.get_title(1);
    assert!(title.is_some());
    assert_eq!(title.unwrap().id, 1);
    let no_title = data.get_title(0);
    assert!(no_title.is_none());
}

#[test]
fn get_all_titles() {
    let data = Data::default().init(Some(31));
    let titles = data.get_all_titles();
    assert!(titles.len() > 0);
}
