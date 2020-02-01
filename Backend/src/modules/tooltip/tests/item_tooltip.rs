use mysql_connection::tools::Execute;

use crate::modules::armory::Armory;
use crate::modules::armory::dto::{CharacterDto, CharacterGearDto, CharacterHistoryDto, CharacterInfoDto, CharacterItemDto};
use crate::modules::armory::tools::SetCharacter;
use crate::modules::data::Data;
use crate::modules::tooltip::domain_value::Stat;
use crate::modules::tooltip::material::SetItem;
use crate::modules::tooltip::tools::RetrieveItemTooltip;
use crate::modules::tooltip::Tooltip;

#[test]
fn avenger_breastplate() {
    let tooltip = Tooltip::default().init();
    let data = Data::default().init(None);

    let result = tooltip.get_item(&data, 1, 1, 21389);
    assert!(result.is_ok());

    let item_tooltip = result.unwrap();
    assert_eq!(item_tooltip.name, "Avenger's Breastplate");
    assert_eq!(item_tooltip.icon, "inv_chest_plate03");
    assert_eq!(item_tooltip.quality, 5);
    assert!(item_tooltip.bonding.contains(&"Binds when picked up".to_string()));
    assert!(item_tooltip.inventory_type.contains(&"Chest".to_string()));
    assert!(item_tooltip.sheath_type.is_none());
    assert!(item_tooltip.sub_class.contains(&"Plate".to_string()));
    assert!(item_tooltip.armor.is_none());
    assert!(item_tooltip.stats.is_some());
    assert_eq!(item_tooltip.stats.as_ref().unwrap().len(), 5);
    assert!(item_tooltip.stats.as_ref().unwrap().contains(&Stat {
        value: 23,
        name: "Strength".to_string(),
    }));
    assert!(item_tooltip.stats.as_ref().unwrap().contains(&Stat {
        value: 12,
        name: "Agility".to_string(),
    }));
    assert!(item_tooltip.stats.as_ref().unwrap().contains(&Stat {
        value: 24,
        name: "Stamina".to_string(),
    }));
    assert!(item_tooltip.stats.as_ref().unwrap().contains(&Stat {
        value: 24,
        name: "Intellect".to_string(),
    }));
    assert!(item_tooltip.stats.as_ref().unwrap().contains(&Stat {
        value: 11,
        name: "Spirit".to_string(),
    }));
    assert!(item_tooltip.durability.contains(&165));
    assert!(item_tooltip.item_level.contains(&88));
    assert!(item_tooltip.required_level.contains(&60));
    assert!(item_tooltip.item_effects.is_some());
    assert_eq!(item_tooltip.item_effects.as_ref().unwrap().len(), 3);
    assert_eq!(item_tooltip.item_effects.as_ref().unwrap()[0], "Increases damage and healing done by magical spells and effects by up to 18.");
    assert_eq!(item_tooltip.item_effects.as_ref().unwrap()[1], "Improves your chance to get a critical strike with spells by 1%.");
    assert_eq!(item_tooltip.item_effects.as_ref().unwrap()[2], "Improves your chance to get a critical strike by 1%.");
    assert!(item_tooltip.socket.is_none());
    assert!(item_tooltip.enchant.is_none());
    assert!(item_tooltip.weapon_stat.is_none());
    assert!(item_tooltip.item_set.is_some());
    assert_eq!(item_tooltip.item_set.as_ref().unwrap().name, "Avenger's Battlegear");
    assert_eq!(item_tooltip.item_set.as_ref().unwrap().set_items.len(), 5);
    assert!(item_tooltip.item_set.as_ref().unwrap().set_items.contains(&SetItem {
        item_id: 21391,
        active: false,
        name: "Avenger's Pauldrons".to_string(),
    }));
    assert!(item_tooltip.item_set.as_ref().unwrap().set_items.contains(&SetItem {
        item_id: 21389,
        active: false,
        name: "Avenger's Breastplate".to_string(),
    }));
    assert!(item_tooltip.item_set.as_ref().unwrap().set_items.contains(&SetItem {
        item_id: 21388,
        active: false,
        name: "Avenger's Greaves".to_string(),
    }));
    assert!(item_tooltip.item_set.as_ref().unwrap().set_items.contains(&SetItem {
        item_id: 21390,
        active: false,
        name: "Avenger's Legguards".to_string(),
    }));
    assert!(item_tooltip.item_set.as_ref().unwrap().set_items.contains(&SetItem {
        item_id: 21387,
        active: false,
        name: "Avenger's Crown".to_string(),
    }));
    assert_eq!(item_tooltip.item_set.as_ref().unwrap().set_effects.len(), 2);
    assert_eq!(item_tooltip.item_set.as_ref().unwrap().set_effects[0].threshold, 3);
    assert_eq!(item_tooltip.item_set.as_ref().unwrap().set_effects[0].description, "Increases the duration of your Judgements by 20%.");
    assert_eq!(item_tooltip.item_set.as_ref().unwrap().set_effects[1].threshold, 5);
    assert_eq!(item_tooltip.item_set.as_ref().unwrap().set_effects[1].description, "Increases damage and healing done by magical spells and effects by up to 71.");
}

#[test]
fn shadowmourne_socket() {
    let tooltip = Tooltip::default().init();
    let data = Data::default().init(None);

    let result = tooltip.get_item(&data, 1, 3, 49623);
    assert!(result.is_ok());

    let item_tooltip = result.unwrap();
    assert!(item_tooltip.socket.is_some());
    assert_eq!(item_tooltip.socket.as_ref().unwrap().socket_bonus, "+8 Strength");
    assert_eq!(item_tooltip.socket.as_ref().unwrap().slots.len(), 3);
    assert_eq!(item_tooltip.socket.as_ref().unwrap().slots.iter().filter(|slot| slot.flag == 2 && slot.item.is_none()).count(), 3);
}

#[test]
fn thunderfury_weapon_stat() {
    let tooltip = Tooltip::default().init();
    let data = Data::default().init(None);

    let result = tooltip.get_item(&data, 1, 1, 19019);
    assert!(result.is_ok());

    let item_tooltip = result.unwrap();
    assert!(item_tooltip.weapon_stat.is_some());
    assert_eq!(item_tooltip.weapon_stat.as_ref().unwrap().delay, 1900);
    assert_eq!(item_tooltip.weapon_stat.as_ref().unwrap().damage_sources.len(), 2);
    assert_eq!(item_tooltip.weapon_stat.as_ref().unwrap().damage_sources[0].damage_min, 44);
    assert_eq!(item_tooltip.weapon_stat.as_ref().unwrap().damage_sources[0].damage_max, 115);
    assert!(item_tooltip.weapon_stat.as_ref().unwrap().damage_sources[0].damage_type.is_none());
    assert_eq!(item_tooltip.weapon_stat.as_ref().unwrap().damage_sources[1].damage_min, 16);
    assert_eq!(item_tooltip.weapon_stat.as_ref().unwrap().damage_sources[1].damage_max, 30);
    assert!(item_tooltip.weapon_stat.as_ref().unwrap().damage_sources[1].damage_type.contains(&"Nature".to_string()));
}

#[test]
fn shadowmourne_socketed_and_enchanted() {
    let tooltip = Tooltip::default().init();
    let data = Data::default().init(None);
    let armory = Armory::default();

    let character_info_dto = CharacterInfoDto {
        gear: CharacterGearDto {
            head: None,
            neck: None,
            shoulder: None,
            back: None,
            chest: None,
            shirt: None,
            tabard: None,
            wrist: None,
            main_hand: Some(CharacterItemDto {
                item_id: 49623,
                random_property_id: None,
                enchant_id: Some(266),
                gem_ids: vec![None, Some(23094), Some(23094), None],
            }),
            off_hand: None,
            ternary_hand: None,
            glove: None,
            belt: None,
            leg: None,
            boot: None,
            ring1: None,
            ring2: None,
            trinket1: None,
            trinket2: None,
        },
        hero_class_id: 1,
        level: 23,
        gender: false,
        profession1: Some(202),
        profession2: None,
        talent_specialization: None,
        race_id: 4,
    };
    let character_history_dto = CharacterHistoryDto {
        character_info: character_info_dto.to_owned(),
        character_name: "sdgsdfsd".to_string(),
        character_title: Some(14),
        profession_skill_points1: Some(75),
        profession_skill_points2: None,
        character_guild: None,
        facial: None
    };
    let character_dto = CharacterDto {
        server_uid: 43356,
        character_history: Some(character_history_dto.to_owned()),
    };

    let character_res = armory.set_character(3, character_dto);
    assert!(character_res.is_ok());
    let character = character_res.unwrap();

    let result = tooltip.get_character_item(&data, &armory, 1, 49623, character.last_update.as_ref().unwrap().id);
    assert!(result.is_ok());

    let item_tooltip = result.unwrap();
    assert!(item_tooltip.enchant.contains(&"Fishing Lure (+100 Fishing Skill)".to_string()));
    assert_eq!(item_tooltip.socket.as_ref().unwrap().socket_bonus, "+8 Strength");
    assert_eq!(item_tooltip.socket.as_ref().unwrap().slots.len(), 3);
    assert_eq!(item_tooltip.socket.as_ref().unwrap().slots.iter().filter(|slot| slot.flag == 2 && slot.item.is_none()).count(), 1);
    assert!(item_tooltip.socket.as_ref().unwrap().slots[0].item.is_none());
    assert!(item_tooltip.socket.as_ref().unwrap().slots[1].item.is_some());
    assert_eq!(item_tooltip.socket.as_ref().unwrap().slots[1].item.as_ref().unwrap().effect, "+7 Spell Power");
    assert!(item_tooltip.socket.as_ref().unwrap().slots[2].item.is_some());
    assert_eq!(item_tooltip.socket.as_ref().unwrap().slots[2].item.as_ref().unwrap().effect, "+7 Spell Power");

    let character_history = character.last_update.unwrap();
    armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.main_hand.unwrap().id));
    armory.db_main.execute_wparams("DELETE FROM armory_gear WHERE id=:id", params!("id" => character_history.character_info.gear.id));
    armory.db_main.execute_wparams("DELETE FROM armory_character_info WHERE id=:id", params!("id" => character_history.character_info.id));
    armory.db_main.execute_wparams("DELETE FROM armory_character_history WHERE id=:id", params!("id" => character_history.id));
    armory.db_main.execute_wparams("DELETE FROM armory_character WHERE id=:id", params!("id" => character_history.character_id));
}