use crate::modules::data::Data;
use crate::modules::data::domain_value::ItemSocket;

pub trait RetrieveItemSocket {
  fn get_item_socket(&self, expansion_id: u8, item_id: u32) -> Option<ItemSocket>;
}

impl RetrieveItemSocket for Data {
  fn get_item_socket(&self, expansion_id: u8, item_id: u32) -> Option<ItemSocket> {
    if expansion_id == 0 {
      return None;
    }

    self.item_sockets.get(expansion_id as usize - 2)
      .and_then(|map| map.get(&item_id)
        .and_then(|item_socket| Some(item_socket.clone())))
  }
}