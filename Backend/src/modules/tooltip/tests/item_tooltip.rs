use crate::modules::tooltip::Tooltip;
use crate::modules::data::Data;
use crate::modules::tooltip::tools::RetrieveItemTooltip;
use crate::modules::tooltip::domain_value::SetItem;

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
  assert_eq!(item_tooltip.stats.as_ref().unwrap()[0].value, 23);
  assert_eq!(item_tooltip.stats.as_ref().unwrap()[0].name, "Strength");
  assert_eq!(item_tooltip.stats.as_ref().unwrap()[1].value, 12);
  assert_eq!(item_tooltip.stats.as_ref().unwrap()[1].name, "Agility");
  assert_eq!(item_tooltip.stats.as_ref().unwrap()[2].value, 24);
  assert_eq!(item_tooltip.stats.as_ref().unwrap()[2].name, "Stamina");
  assert_eq!(item_tooltip.stats.as_ref().unwrap()[3].value, 24);
  assert_eq!(item_tooltip.stats.as_ref().unwrap()[3].name, "Intellect");
  assert_eq!(item_tooltip.stats.as_ref().unwrap()[4].value, 11);
  assert_eq!(item_tooltip.stats.as_ref().unwrap()[4].name, "Spirit");
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
    name: "Avenger's Pauldrons".to_string()
  }));
  assert!(item_tooltip.item_set.as_ref().unwrap().set_items.contains(&SetItem {
    item_id: 21389,
    active: false,
    name: "Avenger's Breastplate".to_string()
  }));
  assert!(item_tooltip.item_set.as_ref().unwrap().set_items.contains(&SetItem {
    item_id: 21388,
    active: false,
    name: "Avenger's Greaves".to_string()
  }));
  assert!(item_tooltip.item_set.as_ref().unwrap().set_items.contains(&SetItem {
    item_id: 21390,
    active: false,
    name: "Avenger's Legguards".to_string()
  }));
  assert!(item_tooltip.item_set.as_ref().unwrap().set_items.contains(&SetItem {
    item_id: 21387,
    active: false,
    name: "Avenger's Crown".to_string()
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