use crate::modules::data::{tools::RetrieveItemsetEffect, Data};
use crate::start_test_db;

#[test]
fn get_itemset_effects() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(30));
    let itemset_effects = data.get_itemset_effects(1, 1);
    assert!(itemset_effects.is_some());
    let itemset_effects_vec = itemset_effects.unwrap();
    assert!(itemset_effects_vec.len() > 0);
    assert_eq!(itemset_effects_vec[0].expansion_id, 1);
    assert_eq!(itemset_effects_vec[0].itemset_id, 1);
    let no_itemset_effects = data.get_itemset_effects(0, 0);
    assert!(no_itemset_effects.is_none());
}
