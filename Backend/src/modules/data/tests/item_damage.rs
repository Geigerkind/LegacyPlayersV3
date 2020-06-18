use crate::modules::data::{tools::RetrieveItemDamage, Data};
use crate::tests::TestContainer;

#[test]
fn get_item_damage() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(20));
    let item_damage = data.get_item_damage(1, 25);
    assert!(item_damage.is_some());
    let unpacked_item_damage_vec = item_damage.unwrap();
    assert!(!unpacked_item_damage_vec.is_empty());
    assert_eq!(unpacked_item_damage_vec[0].item_id, 25);
    assert_eq!(unpacked_item_damage_vec[0].expansion_id, 1);
    let no_item_damage = data.get_item_damage(0, 0);
    assert!(no_item_damage.is_none());
}
