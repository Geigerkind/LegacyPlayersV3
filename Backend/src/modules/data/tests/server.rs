use crate::modules::data::{tools::RetrieveServer, Data};
use crate::tests::TestContainer;

#[test]
fn get_server() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns((dns + "main").as_str()).init(Some(6));
    let server = data.get_server(1);
    assert!(server.is_some());
    assert_eq!(server.unwrap().id, 1);
    let no_server = data.get_server(0);
    assert!(no_server.is_none());
}

#[test]
fn get_all_servers() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns((dns + "main").as_str()).init(Some(6));
    let servers = data.get_all_servers();
    assert!(!servers.is_empty());
}
