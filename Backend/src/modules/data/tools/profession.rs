use crate::modules::data::domain_value::Profession;
use crate::modules::data::Data;

pub trait RetrieveProfession {
  fn get_profession(&self, id: u8) -> Option<Profession>;
  fn get_all_professions(&self) -> Vec<Profession>;
}

impl RetrieveProfession for Data {
  fn get_profession(&self, id: u8) -> Option<Profession> {
    self.professions.get(&id)
      .and_then(|profession| Some(profession.clone()))
  }

  fn get_all_professions(&self) -> Vec<Profession> {
    self.professions.iter().map(|(_, profession)| profession.clone()).collect()
  }
}