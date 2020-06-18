use crate::modules::data::{tools::RetrieveTitle, Data};
use crate::tests::TestContainer;

#[test]
fn get_title() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(31));
    let title = data.get_title(1);
    assert!(title.is_some());
    assert_eq!(title.unwrap().id, 1);
    let no_title = data.get_title(0);
    assert!(no_title.is_none());
}

#[test]
fn get_all_titles() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(31));
    let titles = data.get_all_titles();
    assert!(!titles.is_empty());
}
