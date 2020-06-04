use crate::modules::data::{tools::RetrieveLanguage, Data};
use crate::start_test_db;

#[test]
fn get_language() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(2));
    let language = data.get_language(1);
    assert!(language.is_some());
    assert_eq!(language.unwrap().id, 1);
    let no_language = data.get_language(0);
    assert!(no_language.is_none());
}

#[test]
fn get_all_languages() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(2));
    let languages = data.get_all_languages();
    assert!(languages.len() > 0);
}
