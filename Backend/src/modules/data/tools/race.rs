use crate::modules::data::Data;
use crate::modules::data::domain_value::Race;

pub trait RetrieveRace {
  fn get_race(&self, id: u8) -> Option<Race>;
  fn get_all_races(&self) -> Vec<Race>;
}

impl RetrieveRace for Data {
  fn get_race(&self, id: u8) -> Option<Race> {
    self.races.get(&id)
      .and_then(|race| Some(race.clone()))
  }

  fn get_all_races(&self) -> Vec<Race> {
    self.races.iter().map(|(_, race)| race.clone()).collect()
  }
}