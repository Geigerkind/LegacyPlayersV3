#[cfg(test)]
mod tests {
  use crate::modules::data::Data;
  use crate::modules::data::tools::RetrieveLocalization;

  #[test]
  fn get_localization() {
    let data = Data::default().init();
    let localization = data.get_localization(1, 1);
    assert!(localization.is_some());
    assert_eq!(localization.unwrap(), "Human");
    let no_localization = data.get_localization(0, 0);
    assert!(no_localization.is_none());
  }
}