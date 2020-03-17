use crate::modules::data::{tools::RetrieveHeroClass, Data};

#[test]
fn get_hero_class() {
    let data = Data::default().init(Some(7));
    let hero_class = data.get_hero_class(1);
    assert!(hero_class.is_some());
    assert_eq!(hero_class.unwrap().id, 1);
    let no_hero_class = data.get_hero_class(0);
    assert!(no_hero_class.is_none());
}

#[test]
fn get_all_hero_classs() {
    let data = Data::default().init(Some(7));
    let hero_classes = data.get_all_hero_classes();
    assert!(hero_classes.len() > 0);
}
