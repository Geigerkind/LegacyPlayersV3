use crate::modules::armory::{
    tools::{DeleteCharacterHistory, GetCharacterHistory, SetCharacter, SetCharacterHistory},
    Armory,
};
use std::{thread, time};
use super::helper::get_character_history;
use crate::modules::armory::dto::CharacterDto;
use crate::tests::TestContainer;

#[test]
fn set_character_history() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let armory = Armory::with_dns((dns + "main").as_str());
    let character_dto = CharacterDto { server_uid: 123124, character_history: None };
    let character_history_dto = get_character_history();

    let set_character_res = armory.set_character(3, character_dto.clone());
    assert!(set_character_res.is_ok());

    let set_character = set_character_res.unwrap();
    assert!(set_character.compare_by_value(&character_dto));

    let set_character_history_res = armory.set_character_history(3, character_history_dto.clone(), set_character.server_uid);
    assert!(set_character_history_res.is_ok());

    let set_character_history = set_character_history_res.unwrap();
    assert!(set_character_history.compare_by_value(&character_history_dto));

    let character_history_res = armory.get_character_history(set_character_history.id);
    assert!(character_history_res.is_ok());

    let character_history = character_history_res.unwrap();
    println!("{:?}", set_character_history.arena_teams);
    println!("{:?}", character_history.arena_teams);
    assert!(character_history.deep_eq(&set_character_history));

    // Sleeping 2 seconds in order to cause an timestamp update in the DB
    thread::sleep(time::Duration::from_millis(2000));
    let set_character_history_res2 = armory.set_character_history(3, character_history_dto, set_character.server_uid);
    assert!(set_character_history_res2.is_ok());

    let set_character_history2 = set_character_history_res2.unwrap();
    assert!(set_character_history2.timestamp >= set_character_history.timestamp + 2);

    let character_history_res2 = armory.get_character_history(set_character_history2.id);
    assert!(character_history_res2.is_ok());

    let character_history2 = character_history_res2.unwrap();
    assert!(character_history2.deep_eq(&set_character_history2));

    let delete_character_history = armory.delete_character_history(character_history.id);
    assert!(delete_character_history.is_ok());

    let character_history_res3 = armory.get_character_history(character_history.id);
    assert!(character_history_res3.is_err());
}
