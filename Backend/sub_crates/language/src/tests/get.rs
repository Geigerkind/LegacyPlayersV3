#[cfg(test)]
mod tests {
  use crate::domain_value::Language;
  use crate::material::Dictionary;
  use crate::tools::Get;
  use crate::tools::Register;

  #[test]
  #[should_panic]
  fn key_not_registered() {
    let dictionary = Dictionary::default();
    dictionary.get("Test", Language::English);
  }

  #[test]
  #[should_panic]
  fn language_not_registered() {
    let dictionary = Dictionary::default();
    dictionary.register("Test", Language::English, "test");
    dictionary.get("Test", Language::German);
  }

  #[test]
  #[should_panic]
  fn language_not_registered_because_empty() {
    let dictionary = Dictionary::default();
    dictionary.register("Test", Language::Japanese, "test");
    dictionary.get("Test", Language::English);
  }

  #[test]
  fn language_exists() {
    let dictionary = Dictionary::default();
    dictionary.register("Test", Language::Japanese, "test");
    assert_eq!(dictionary.get("Test", Language::Japanese), "test")
  }
}