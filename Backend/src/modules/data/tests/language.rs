#[cfg(test)]
mod tests {
  use crate::modules::data::Data;
  use crate::modules::data::tools::RetrieveLanguage;

  #[test]
  fn get_expansion() {
    let data = Data::default().init();
    let language = data.get_language(1);
    assert!(language.is_some());
    assert_eq!(language.unwrap().id, 1);
    let no_language = data.get_language(0);
    assert!(no_language.is_none());
  }

  #[test]
  fn get_all_expansions() {
    let data = Data::default().init();
    let languages = data.get_all_languages();
    assert!(languages.len() > 0);
  }
}