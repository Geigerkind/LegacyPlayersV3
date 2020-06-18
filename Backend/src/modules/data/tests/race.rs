use crate::modules::data::{tools::RetrieveRace, Data};
use crate::tests::TestContainer;

#[test]
fn get_race() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(4));
    let race = data.get_race(1);
    assert!(race.is_some());
    assert_eq!(race.unwrap().id, 1);
    let no_race = data.get_race(0);
    assert!(no_race.is_none());
}

#[test]
fn get_all_races() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(4));
    let races = data.get_all_races();
    assert!(!races.is_empty());
}
