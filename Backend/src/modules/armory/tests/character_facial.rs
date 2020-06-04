use crate::modules::armory::{
    tools::{CreateCharacterFacial, GetCharacterFacial},
    Armory,
};
use super::helper::get_character_facial;
use crate::start_test_db;

#[test]
fn character_facial() {
    let dns: String;
    start_test_db!(false, dns);

    let armory = Armory::with_dns((dns + "main").as_str());
    let character_facial_dto = get_character_facial();

    let character_facial_res = armory.create_character_facial(character_facial_dto.clone());
    assert!(character_facial_res.is_ok());

    let character_facial = character_facial_res.unwrap();
    assert!(character_facial.compare_by_value(&character_facial_dto));

    let character_facial2_res = armory.get_character_facial(character_facial.id);
    assert!(character_facial2_res.is_ok());

    let character_facial2 = character_facial2_res.unwrap();
    assert!(character_facial2.deep_eq(&character_facial));
}
