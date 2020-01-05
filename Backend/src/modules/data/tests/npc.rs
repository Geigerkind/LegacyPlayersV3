#[cfg(test)]
mod tests {
  use crate::modules::data::Data;
  use crate::modules::data::tools::RetrieveNPC;

  #[test]
  fn get_npc() {
    let data = Data::default().init();
    let npc = data.get_npc(1, 1);
    assert!(npc.is_some());
    let unpacked_npc = npc.unwrap();
    assert_eq!(unpacked_npc.id, 1);
    assert_eq!(unpacked_npc.expansion_id, 1);
    let no_npc = data.get_npc(0, 0);
    assert!(no_npc.is_none());
  }
}