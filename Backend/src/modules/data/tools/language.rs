use crate::modules::data::domain_value::Language;
use crate::modules::data::Data;

pub trait RetrieveLanguage {
  fn get_language(&self, id: u8) -> Option<Language>;
  fn get_all_languages(&self) -> Vec<Language>;
}

impl RetrieveLanguage for Data {
  fn get_language(&self, id: u8) -> Option<Language> {
    self.languages.get(&id)
      .and_then(|language| Some(language.clone()))
  }

  fn get_all_languages(&self) -> Vec<Language> {
    self.languages.iter().map(|(_, language)| language.clone()).collect()
  }
}