#[cfg(test)]
mod tests {
  use crate::modules::data::Data;
  use crate::modules::data::tools::RetrieveGem;

  #[test]
  fn get_gem() {
    let data = Data::default().init(Some(16));
    let gem = data.get_gem(2, 22459);
    assert!(gem.is_some());
    let unpacked_gem = gem.unwrap();
    assert_eq!(unpacked_gem.item_id, 22459);
    assert_eq!(unpacked_gem.expansion_id, 2);
    let no_gem = data.get_gem(0, 0);
    assert!(no_gem.is_none());
  }
}