use crate::modules::data::{tools::RetrieveItemsetEffect, Data};
use crate::tests::TestContainer;

#[test]
fn get_itemset_effects() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns((dns + "main").as_str()).init(Some(30));
    let itemset_effects = data.get_itemset_effects(1, 1);
    assert!(itemset_effects.is_some());
    let itemset_effects_vec = itemset_effects.unwrap();
    assert!(!itemset_effects_vec.is_empty());
    assert_eq!(itemset_effects_vec[0].expansion_id, 1);
    assert_eq!(itemset_effects_vec[0].itemset_id, 1);
    let no_itemset_effects = data.get_itemset_effects(0, 0);
    assert!(no_itemset_effects.is_none());
}
