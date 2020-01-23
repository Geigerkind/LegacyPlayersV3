use crate::modules::data::Data;
use crate::modules::data::tools::RetrieveProfession;

#[test]
fn get_profession() {
  let data = Data::default().init(Some(5));
  let profession = data.get_profession(182);
  assert!(profession.is_some());
  assert_eq!(profession.unwrap().id, 182);
  let no_profession = data.get_profession(0);
  assert!(no_profession.is_none());
}

#[test]
fn get_all_professions() {
  let data = Data::default().init(Some(5));
  let professions = data.get_all_professions();
  assert!(professions.len() > 0);
}