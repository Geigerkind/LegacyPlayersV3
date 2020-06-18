use crate::modules::data::{tools::RetrieveDispelType, Data};
use crate::tests::TestContainer;

#[test]
fn get_dispel_type() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(9));
    let dispel_type = data.get_dispel_type(1);
    assert!(dispel_type.is_some());
    assert_eq!(dispel_type.unwrap().id, 1);
    let no_dispel_type = data.get_dispel_type(0);
    assert!(no_dispel_type.is_none());
}

#[test]
fn get_all_dispel_types() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(9));
    let dispel_types = data.get_all_dispel_types();
    assert!(!dispel_types.is_empty());
}
