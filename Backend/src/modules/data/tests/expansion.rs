use crate::modules::data::Data;
use crate::modules::data::tools::RetrieveExpansion;

#[test]
fn get_expansion() {
  let data = Data::default().init(Some(1));
  let expansion = data.get_expansion(1);
  assert!(expansion.is_some());
  assert_eq!(expansion.unwrap().id, 1);
  let no_expansion = data.get_expansion(0);
  assert!(no_expansion.is_none());
}

#[test]
fn get_all_expansions() {
  let data = Data::default().init(Some(1));
  let expansions = data.get_all_expansions();
  assert!(expansions.len() > 0);
}