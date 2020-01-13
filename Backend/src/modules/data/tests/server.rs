use crate::modules::data::Data;
use crate::modules::data::tools::RetrieveServer;

#[test]
fn get_server() {
  let data = Data::default().init(Some(6));
  let server = data.get_server(1);
  assert!(server.is_some());
  assert_eq!(server.unwrap().id, 1);
  let no_server = data.get_server(0);
  assert!(no_server.is_none());
}

#[test]
fn get_all_servers() {
  let data = Data::default().init(Some(6));
  let servers = data.get_all_servers();
  assert!(servers.len() > 0);
}