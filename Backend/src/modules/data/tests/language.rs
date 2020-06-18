use crate::modules::data::{tools::RetrieveLanguage, Data};
use crate::tests::TestContainer;

#[test]
fn get_language() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(2));
    let language = data.get_language(1);
    assert!(language.is_some());
    assert_eq!(language.unwrap().id, 1);
    let no_language = data.get_language(0);
    assert!(no_language.is_none());
}

#[test]
fn get_all_languages() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(2));
    let languages = data.get_all_languages();
    assert!(!languages.is_empty());
}
