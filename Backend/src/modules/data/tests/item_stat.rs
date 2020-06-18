use crate::modules::data::{tools::RetrieveItemStat, Data};
use crate::tests::TestContainer;

#[test]
fn get_item_stats() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(28));
    let item_stats = data.get_item_stats(1, 940);
    assert!(item_stats.is_some());
    let item_stats_vec = item_stats.unwrap();
    assert!(!item_stats_vec.is_empty());
    assert_eq!(item_stats_vec[0].expansion_id, 1);
    assert_eq!(item_stats_vec[0].item_id, 940);
    let no_item_stats = data.get_item_stats(0, 0);
    assert!(no_item_stats.is_none());
}
