use crate::modules::data::Data;
use crate::modules::data::tools::RetrieveItemEffect;

#[test]
fn get_item_effect() {
  let data = Data::default().init(Some(22));
  let item_effect = data.get_item_effect(1, 117);
  assert!(item_effect.is_some());
  let unpacked_item_effect_vec = item_effect.unwrap();
  assert!(unpacked_item_effect_vec.len() > 0);
  assert_eq!(unpacked_item_effect_vec[0].item_id, 117);
  assert_eq!(unpacked_item_effect_vec[0].expansion_id, 1);
  let no_item_effect = data.get_item_effect(0, 0);
  assert!(no_item_effect.is_none());
}