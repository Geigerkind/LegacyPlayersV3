use crate::modules::data::domain_value::Title;
use crate::modules::data::Data;

pub trait RetrieveTitle {
    fn get_title(&self, id: u16) -> Option<Title>;
    fn get_all_titles(&self) -> Vec<Title>;
}

impl RetrieveTitle for Data {
    fn get_title(&self, id: u16) -> Option<Title> {
        self.titles.get(&id).and_then(|title| Some(title.clone()))
    }

    fn get_all_titles(&self) -> Vec<Title> {
        self.titles.iter().map(|(_, title)| title.clone()).collect()
    }
}
