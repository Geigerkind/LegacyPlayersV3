use crate::modules::data::{tools::RetrieveNPC, Data};
use crate::tests::TestContainer;

#[test]
fn get_npc() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(13));
    let npc = data.get_npc(1, 1);
    assert!(npc.is_some());
    let unpacked_npc = npc.unwrap();
    assert_eq!(unpacked_npc.id, 1);
    assert_eq!(unpacked_npc.expansion_id, 1);
    let no_npc = data.get_npc(0, 0);
    assert!(no_npc.is_none());
}
