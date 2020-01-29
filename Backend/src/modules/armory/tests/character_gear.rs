use crate::modules::armory::dto::{CharacterGearDto, CharacterItemDto};
use crate::modules::armory::Armory;
use crate::modules::armory::tools::{CreateCharacterGear, GetCharacterGear};
use mysql_connection::tools::Execute;

#[test]
fn character_gear() {
  let armory = Armory::default();
  let character_gear_dto = CharacterGearDto {
    head: Some(CharacterItemDto {
      item_id: 32235,
      random_property_id: Some(-12),
      random_property_scaling_factor: Some(666),
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    neck: Some(CharacterItemDto {
      item_id: 30022,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    shoulder: Some(CharacterItemDto {
      item_id: 30055,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    back: Some(CharacterItemDto {
      item_id: 33590,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    chest: Some(CharacterItemDto {
      item_id: 30990,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    shirt: None,
    tabard: None,
    wrist: Some(CharacterItemDto {
      item_id: 32574,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    main_hand: Some(CharacterItemDto {
      item_id: 32332,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    off_hand: None,
    ternary_hand: Some(CharacterItemDto {
      item_id: 27484,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    glove: Some(CharacterItemDto {
      item_id: 32347,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    belt: Some(CharacterItemDto {
      item_id: 30879,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    leg: Some(CharacterItemDto {
      item_id: 32341,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    boot: Some(CharacterItemDto {
      item_id: 32345,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    ring1: Some(CharacterItemDto {
      item_id: 33496,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    ring2: Some(CharacterItemDto {
      item_id: 32335,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    trinket1: Some(CharacterItemDto {
      item_id: 28830,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    }),
    trinket2: Some(CharacterItemDto {
      item_id: 29383,
      random_property_id: None,
      random_property_scaling_factor: None,
      enchant_id: None,
      gem_ids: vec![None, None, None, None]
    })
  };

  let character_gear_res = armory.create_character_gear(character_gear_dto.clone());
  assert!(character_gear_res.is_ok());

  let character_gear = character_gear_res.unwrap();
  assert!(character_gear.compare_by_value(&character_gear_dto));

  let character_gear_res2 = armory.get_character_gear(character_gear.id);
  assert!(character_gear_res2.is_ok());

  let character_gear2 = character_gear_res2.unwrap();
  assert!(character_gear2.deep_eq(&character_gear));

  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.head.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.neck.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.shoulder.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.back.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.chest.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.wrist.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.main_hand.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.ternary_hand.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.glove.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.belt.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.leg.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.boot.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.ring1.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.ring2.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.trinket1.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_gear.trinket2.unwrap().id));
  armory.db_main.execute_wparams("DELETE FROM armory_gear WHERE id=:id", params!("id" => character_gear.id));
}