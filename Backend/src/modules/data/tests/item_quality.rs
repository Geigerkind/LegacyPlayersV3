use crate::modules::data::{tools::RetrieveItemQuality, Data};
use crate::tests::TestContainer;

#[test]
fn get_item_quality() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(24));
    let item_quality = data.get_item_quality(1);
    assert!(item_quality.is_some());
    assert_eq!(item_quality.unwrap().id, 1);
    let no_item_quality = data.get_item_quality(0);
    assert!(no_item_quality.is_none());
}

#[test]
fn get_all_item_qualities() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(24));
    let item_qualities = data.get_all_item_qualities();
    assert!(!item_qualities.is_empty());
}
