use crate::modules::data::{tools::RetrieveRace, Data};
use crate::start_test_db;

#[test]
fn get_race() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(4));
    let race = data.get_race(1);
    assert!(race.is_some());
    assert_eq!(race.unwrap().id, 1);
    let no_race = data.get_race(0);
    assert!(no_race.is_none());
}

#[test]
fn get_all_races() {
    let dns: String;
    start_test_db!(true, dns);

    let data = Data::with_dns((dns + "main").as_str()).init(Some(4));
    let races = data.get_all_races();
    assert!(races.len() > 0);
}
