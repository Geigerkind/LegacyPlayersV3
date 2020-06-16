use crate::modules::data::{tools::RetrieveProfession, Data};
use crate::tests::TestContainer;

#[test]
fn get_profession() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns((dns + "main").as_str()).init(Some(5));
    let profession = data.get_profession(182);
    assert!(profession.is_some());
    assert_eq!(profession.unwrap().id, 182);
    let no_profession = data.get_profession(0);
    assert!(no_profession.is_none());
}

#[test]
fn get_all_professions() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns((dns + "main").as_str()).init(Some(5));
    let professions = data.get_all_professions();
    assert!(professions.len() > 0);
}
