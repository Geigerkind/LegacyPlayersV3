use crate::modules::armory::dto::{CharacterGearDto, CharacterInfoDto, CharacterItemDto};
use crate::modules::armory::tools::{CreateCharacterInfo, GetCharacterInfo};
use crate::modules::armory::Armory;
use mysql_connection::tools::Execute;

#[test]
fn character_info() {
    let armory = Armory::default();
    let character_info_dto = CharacterInfoDto {
        gear: CharacterGearDto {
            head: Some(CharacterItemDto {
                item_id: 31012,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            neck: Some(CharacterItemDto {
                item_id: 33281,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            shoulder: Some(CharacterItemDto {
                item_id: 31022,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            back: Some(CharacterItemDto {
                item_id: 33592,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            chest: Some(CharacterItemDto {
                item_id: 31016,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            shirt: None,
            tabard: None,
            wrist: Some(CharacterItemDto {
                item_id: 32577,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            main_hand: Some(CharacterItemDto {
                item_id: 32500,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            off_hand: Some(CharacterItemDto {
                item_id: 30882,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            ternary_hand: Some(CharacterItemDto {
                item_id: 28523,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            glove: Some(CharacterItemDto {
                item_id: 32328,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            belt: Some(CharacterItemDto {
                item_id: 32258,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            leg: Some(CharacterItemDto {
                item_id: 31019,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            boot: Some(CharacterItemDto {
                item_id: 33324,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            ring1: Some(CharacterItemDto {
                item_id: 32528,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            ring2: Some(CharacterItemDto {
                item_id: 33498,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            trinket1: Some(CharacterItemDto {
                item_id: 13503,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            trinket2: Some(CharacterItemDto {
                item_id: 32496,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
        },
        hero_class_id: 2,
        level: 70,
        gender: true,
        profession1: None,
        profession2: Some(333),
        talent_specialization: None,
        race_id: 3,
    };

    let character_info_res = armory.create_character_info(character_info_dto.clone());
    assert!(character_info_res.is_ok());

    let character_info = character_info_res.unwrap();
    assert!(character_info.compare_by_value(&character_info_dto));

    let character_info_res2 = armory.get_character_info(character_info.id);
    assert!(character_info_res2.is_ok());

    let character_info2 = character_info_res2.unwrap();
    assert!(character_info2.deep_eq(&character_info));

    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.head.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.neck.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.shoulder.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.back.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.chest.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.wrist.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.main_hand.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.off_hand.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.ternary_hand.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.glove.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.belt.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.leg.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.boot.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.ring1.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.ring2.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.trinket1.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_item WHERE id=:id",
        params!("id" => character_info.gear.trinket2.unwrap().id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_gear WHERE id=:id",
        params!("id" => character_info.gear.id),
    );
    armory.db_main.execute_wparams(
        "DELETE FROM armory_character_info WHERE id=:id",
        params!("id" => character_info.id),
    );
}
