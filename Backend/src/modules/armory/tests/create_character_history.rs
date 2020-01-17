use crate::modules::armory::Armory;
use crate::modules::armory::dto::{CharacterDto, CharacterGearDto, CharacterHistoryDto, CharacterInfoDto, CharacterItemDto, GuildDto};
use crate::modules::armory::tools::{SetCharacterHistory, SetCharacter, GetCharacterHistory};
use mysql_connection::tools::Execute;
use std::{time, thread};

#[test]
fn set_character_history() {
  let armory = Armory::default().init();
  let character_dto = CharacterDto {
    server_uid: 123124,
    character_history: None,
  };
  let character_info_dto = CharacterInfoDto {
    gear: CharacterGearDto {
      head: Some(CharacterItemDto {
        item_id: 46151,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      neck: Some(CharacterItemDto {
        item_id: 47988,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      shoulder: Some(CharacterItemDto {
        item_id: 48395,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      back: Some(CharacterItemDto {
        item_id: 48668,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      chest: Some(CharacterItemDto {
        item_id: 48391,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      shirt: None,
      tabard: None,
      wrist: Some(CharacterItemDto {
        item_id: 47573,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      main_hand: Some(CharacterItemDto {
        item_id: 45516,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      off_hand: Some(CharacterItemDto {
        item_id: 45516,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      ternary_hand: Some(CharacterItemDto {
        item_id: 47883,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      glove: Some(CharacterItemDto {
        item_id: 47492,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      belt: Some(CharacterItemDto {
        item_id: 46095,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      leg: Some(CharacterItemDto {
        item_id: 46150,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      boot: Some(CharacterItemDto {
        item_id: 47312,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      ring1: Some(CharacterItemDto {
        item_id: 46322,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      ring2: Some(CharacterItemDto {
        item_id: 45157,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      trinket1: Some(CharacterItemDto {
        item_id: 42987,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
      trinket2: Some(CharacterItemDto {
        item_id: 45931,
        random_property_id: None,
        enchant_id: None,
        gem_ids: vec![None, None, None, None],
      }),
    },
    hero_class_id: 5,
    level: 80,
    gender: true,
    profession1: Some(3),
    profession2: None,
    talent_specialization: None,
    faction: false,
    race_id: 6,
  };
  let character_history_dto = CharacterHistoryDto {
    character_uid: 123124,
    character_info: character_info_dto.to_owned(),
    character_name: "Peterpansi".to_string(),
    guild: Some(GuildDto {
      name: "SampleGuild".to_string(),
      server_uid: 1234,
    }),
    guild_rank: Some("Raider".to_string()),
  };

  let set_character_res = armory.set_character(3, character_dto.clone());
  assert!(set_character_res.is_ok());

  let set_character = set_character_res.unwrap();
  assert!(set_character.compare_by_value(&character_dto));

  let set_character_history_res = armory.set_character_history(3, character_history_dto.clone());
  assert!(set_character_history_res.is_ok());

  let set_character_history = set_character_history_res.unwrap();
  assert!(set_character_history.compare_by_value(&character_history_dto));

  let character_history_res = armory.get_character_history(set_character_history.id);
  assert!(character_history_res.is_ok());

  let character_history = character_history_res.unwrap();
  assert!(character_history.deep_eq(&set_character_history));

  // Sleeping 2 seconds in order to cause an timestamp update in the DB
  thread::sleep(time::Duration::from_millis(2000));
  let set_character_history_res2 = armory.set_character_history(3, character_history_dto.clone());
  assert!(set_character_history_res2.is_ok());

  let set_character_history2 = set_character_history_res2.unwrap();
  assert!(set_character_history2.timestamp >= set_character_history.timestamp + 2);

  let character_history_res2 = armory.get_character_history(set_character_history2.id);
  assert!(character_history_res2.is_ok());

  let character_history2 = character_history_res2.unwrap();
  assert!(character_history2.deep_eq(&set_character_history2));

  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.head.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.neck.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.shoulder.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.back.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.chest.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.wrist.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.main_hand.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.off_hand.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.ternary_hand.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.glove.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.belt.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.leg.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.boot.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.ring1.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.ring2.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.trinket1.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.trinket2.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_gear WHERE id=:id", params!("id" => character_history.character_info.gear.id));
  armory.db_main.execute_wparams("DELETE FROM armory_character_info WHERE id=:id", params!("id" => character_history.character_info.id));
  armory.db_main.execute_wparams("DELETE FROM armory_character_history WHERE id=:id", params!("id" => character_history.id));
  armory.db_main.execute_wparams("DELETE FROM armory_character WHERE id=:id", params!("id" => character_history.character_id));
}