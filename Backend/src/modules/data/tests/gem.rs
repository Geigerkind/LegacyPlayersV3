use crate::modules::data::{tools::RetrieveGem, Data};
use crate::tests::TestContainer;

#[test]
fn get_gem() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns((dns + "main").as_str()).init(Some(16));
    let gem = data.get_gem(2, 22459);
    assert!(gem.is_some());
    let unpacked_gem = gem.unwrap();
    assert_eq!(unpacked_gem.item_id, 22459);
    assert_eq!(unpacked_gem.expansion_id, 2);
    let no_gem = data.get_gem(0, 0);
    assert!(no_gem.is_none());
}
