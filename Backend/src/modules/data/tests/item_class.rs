use crate::modules::data::{tools::RetrieveItemClass, Data};
use crate::tests::TestContainer;

#[test]
fn get_item_class() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns((dns + "main").as_str()).init(Some(19));
    let item_class = data.get_item_class(1);
    assert!(item_class.is_some());
    assert_eq!(item_class.unwrap().id, 1);
    let no_item_class = data.get_item_class(0);
    assert!(no_item_class.is_none());
}

#[test]
fn get_all_item_classes() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns((dns + "main").as_str()).init(Some(19));
    let item_classs = data.get_all_item_classes();
    assert!(!item_classs.is_empty());
}
