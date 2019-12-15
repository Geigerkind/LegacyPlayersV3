use crate::domain_value::Language;
use crate::material::Dictionary;

pub trait Get {
  fn get(&self, key: &str, language: Language) -> String;
}

impl Get for Dictionary {
  fn get(&self, key: &str, language: Language) -> String
  {
    let lang_table = self.table.read().unwrap();
    let lang_index = language as usize;
    match lang_table.get(key) {
      Some(translations) => {
        match translations.get(lang_index) {
          Some(value) => {
            if value.is_empty() {
              panic!("Value is empty for key {} and language {}", key, lang_index)
            } else {
              return value.to_string();
            }
          }
          None => panic!("Key {} is not registered for language {}", key, lang_index)
        }
      }
      None => panic!("Key {} is not registered", key)
    }
  }
}