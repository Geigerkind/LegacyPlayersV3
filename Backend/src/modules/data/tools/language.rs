use crate::modules::data::domain_value::Language;
use crate::modules::data::Data;

pub trait RetrieveLanguage {
    fn get_language(&self, id: u8) -> Option<Language>;
    fn get_language_by_short_code(&self, short_code: String) -> Option<Language>;
    fn get_all_languages(&self) -> Vec<Language>;
}

impl RetrieveLanguage for Data {
    fn get_language(&self, id: u8) -> Option<Language> {
        self.languages
            .get(&id)
            .and_then(|language| Some(language.clone()))
    }

    fn get_language_by_short_code(&self, short_code: String) -> Option<Language> {
        let lang_short_code = short_code.to_lowercase();
        self.languages
            .iter()
            .find(|(_, language)| language.short_code.to_lowercase() == lang_short_code)
            .and_then(|(_, language)| Some(language.clone()))
    }

    fn get_all_languages(&self) -> Vec<Language> {
        self.languages
            .iter()
            .map(|(_, language)| language.clone())
            .collect()
    }
}
