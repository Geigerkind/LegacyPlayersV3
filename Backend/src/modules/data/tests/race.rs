#[cfg(test)]
mod tests {
  use crate::modules::data::Data;
  use crate::modules::data::tools::RetrieveRace;

  #[test]
  fn get_race() {
    let data = Data::default().init(Some(4));
    let race = data.get_race(1);
    assert!(race.is_some());
    assert_eq!(race.unwrap().id, 1);
    let no_race = data.get_race(0);
    assert!(no_race.is_none());
  }

  #[test]
  fn get_all_races() {
    let data = Data::default().init(Some(4));
    let races = data.get_all_races();
    assert!(races.len() > 0);
  }
}