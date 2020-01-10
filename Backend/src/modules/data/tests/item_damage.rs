#[cfg(test)]
mod tests {
  use crate::modules::data::Data;
  use crate::modules::data::tools::RetrieveItemDamage;

  #[test]
  fn get_item_damage() {
    let data = Data::default().init(Some(20));
    let item_damage = data.get_item_damage(1, 25);
    assert!(item_damage.is_some());
    let unpacked_item_damage = item_damage.unwrap();
    assert_eq!(unpacked_item_damage.item_id, 25);
    assert_eq!(unpacked_item_damage.expansion_id, 1);
    let no_item_damage = data.get_item_damage(0, 0);
    assert!(no_item_damage.is_none());
  }
}