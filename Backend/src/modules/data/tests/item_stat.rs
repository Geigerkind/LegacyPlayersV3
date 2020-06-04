use crate::modules::data::{tools::RetrieveItemStat, Data};
use crate::start_test_db;

#[test]
fn get_item_stats() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(28));
    let item_stats = data.get_item_stats(1, 940);
    assert!(item_stats.is_some());
    let item_stats_vec = item_stats.unwrap();
    assert!(item_stats_vec.len() > 0);
    assert_eq!(item_stats_vec[0].expansion_id, 1);
    assert_eq!(item_stats_vec[0].item_id, 940);
    let no_item_stats = data.get_item_stats(0, 0);
    assert!(no_item_stats.is_none());
}
