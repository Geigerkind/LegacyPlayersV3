#[cfg(test)]
mod tests {
  use crate::modules::data::Data;
  use crate::modules::data::tools::RetrieveDispelType;

  #[test]
  fn get_dispel_type() {
    let data = Data::default().init();
    let dispel_type = data.get_dispel_type(1);
    assert!(dispel_type.is_some());
    assert_eq!(dispel_type.unwrap().id, 1);
    let no_dispel_type = data.get_dispel_type(0);
    assert!(no_dispel_type.is_none());
  }

  #[test]
  fn get_all_dispel_types() {
    let data = Data::default().init();
    let dispel_types = data.get_all_dispel_types();
    assert!(dispel_types.len() > 0);
  }
}