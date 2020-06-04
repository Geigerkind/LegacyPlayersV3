use crate::modules::data::{tools::RetrieveNPC, Data};
use crate::start_test_db;

#[test]
fn get_npc() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(13));
    let npc = data.get_npc(1, 1);
    assert!(npc.is_some());
    let unpacked_npc = npc.unwrap();
    assert_eq!(unpacked_npc.id, 1);
    assert_eq!(unpacked_npc.expansion_id, 1);
    let no_npc = data.get_npc(0, 0);
    assert!(no_npc.is_none());
}
