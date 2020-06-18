use crate::modules::data::{tools::RetrieveItemSocket, Data};
use crate::tests::TestContainer;

#[test]
fn get_item_socket() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(27));
    let item_socket = data.get_item_socket(2, 21846);
    assert!(item_socket.is_some());
    let unpacked_item_socket = item_socket.unwrap();
    assert_eq!(unpacked_item_socket.item_id, 21846);
    assert_eq!(unpacked_item_socket.expansion_id, 2);
    let no_item_socket = data.get_item_socket(0, 0);
    assert!(no_item_socket.is_none());
}
