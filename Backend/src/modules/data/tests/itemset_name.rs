use crate::modules::data::{tools::RetrieveItemsetName, Data};
use crate::tests::TestContainer;

#[test]
fn get_itemset_name() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(29));
    let itemset_name = data.get_itemset_name(1, 1);
    assert!(itemset_name.is_some());
    let unpacked_itemset_name = itemset_name.unwrap();
    assert_eq!(unpacked_itemset_name.id, 1);
    assert_eq!(unpacked_itemset_name.expansion_id, 1);
    let no_itemset_name = data.get_itemset_name(0, 0);
    assert!(no_itemset_name.is_none());
}
