use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::CharacterItem;
use crate::modules::armory::tools::GetCharacterHistory;
use crate::modules::data::Data;
use crate::modules::data::tools::{RetrieveEnchant, RetrieveGem, RetrieveIcon, RetrieveItem, RetrieveItemBonding, RetrieveItemClass, RetrieveItemDamage, RetrieveItemDamageType, RetrieveItemEffect, RetrieveItemInventoryType, RetrieveItemsetEffect, RetrieveItemsetName, RetrieveItemSheath, RetrieveItemSocket, RetrieveItemStat, RetrieveLocalization, RetrieveStatType, SpellDescription, RetrieveItemRandomProperty, RetrieveItemRandomPropertyPoints};
use crate::modules::tooltip::domain_value::{ItemSet, SetEffect, SocketSlot, SocketSlotItem, Stat, WeaponDamage, WeaponStat};
use crate::modules::tooltip::dto::TooltipFailure;
use crate::modules::tooltip::material::{ItemTooltip, SetItem, Socket};
use crate::modules::tooltip::Tooltip;
use crate::modules::tooltip::tools::RetrieveCharacterTooltip;

pub trait RetrieveItemTooltip {
  fn get_item(&self, data: &Data, language_id: u8, expansion_id: u8, item_id: u32) -> Result<ItemTooltip, TooltipFailure>;
  fn get_character_item(&self, data: &Data, armory: &Armory, language_id: u8, item_id: u32, character_history_id: u32) -> Result<ItemTooltip, TooltipFailure>;
}

impl RetrieveItemTooltip for Tooltip {
  fn get_item(&self, data: &Data, language_id: u8, expansion_id: u8, item_id: u32) -> Result<ItemTooltip, TooltipFailure> {
    let item_res = data.get_item(expansion_id, item_id);
    if item_res.is_none() {
      return Err(TooltipFailure::InvalidInput);
    }
    let item = item_res.unwrap();

    let mut weapon_stat = None;
    if item.delay.is_some() {
      weapon_stat = Some(WeaponStat {
        delay: item.delay.unwrap(),
        damage_sources: data.get_item_damage(expansion_id, item_id).unwrap().iter().map(|item_damage| WeaponDamage {
          damage_min: item_damage.dmg_min,
          damage_max: item_damage.dmg_max,
          damage_type: item_damage.dmg_type.and_then(|damage_type_id| data.get_item_damage_type(damage_type_id).and_then(|item_damage_type| data.get_localization(language_id, item_damage_type.localization_id).and_then(|localization| Some(localization.content)))),
        }).collect(),
      });
    }

    let socket = data.get_item_socket(expansion_id, item_id)
      .and_then(|item_socket| Some(Socket {
        socket_bonus: data.get_enchant(expansion_id, item_socket.bonus).and_then(|enchant| data.get_localization(language_id, enchant.localization_id).and_then(|localization| Some(localization.content))).unwrap(),
        slots: item_socket.slots.iter().map(|slot_flag| SocketSlot {
          flag: slot_flag.to_owned(),
          item: None,
        }).collect(),
      }));

    let item_stats = data.get_item_stats(expansion_id, item_id);
    let stats: Option<Vec<Stat>> = item_stats.as_ref()
      .and_then(|inner_item_stats| Some(inner_item_stats.iter().map(|item_stat| Stat {
        value: item_stat.stat.stat_value,
        name: data.get_stat_type(item_stat.stat.stat_type).and_then(|stat_type| data.get_localization(language_id, stat_type.localization_id).and_then(|localization| Some(localization.content))).unwrap(),
      }).collect()));

    let armor = item_stats.and_then(|inner_item_stats| inner_item_stats.iter().find(|item_stat| item_stat.stat.stat_type == 34).and_then(|armor| Some(armor.stat.stat_value)));

    let mut item_set = item.itemset.and_then(|itemset_id| data.get_itemset_name(expansion_id, itemset_id).and_then(|itemset_name| Some(ItemSet {
      name: data.get_localization(language_id, itemset_name.localization_id).and_then(|localization| Some(localization.content)).unwrap(),
      set_items: data.get_itemset_item_ids(expansion_id, itemset_id).unwrap().iter().map(|item_id| SetItem {
        item_id: item_id.clone(),
        active: false,
        name: data.get_item(expansion_id, *item_id).and_then(|item| data.get_localization(language_id, item.localization_id).and_then(|localization| Some(localization.content))).unwrap(),
      }).collect(),
      set_effects: data.get_itemset_effects(expansion_id, itemset_id).unwrap().iter().map(|itemset_effect| SetEffect {
        threshold: itemset_effect.threshold,
        description: data.get_localized_spell_description(expansion_id, language_id, itemset_effect.spell_id).unwrap(),
      }).collect::<Vec<SetEffect>>(),
    })));

    if item_set.is_some() {
      item_set.as_mut().unwrap().set_effects.sort_by(|left, right| left.threshold.cmp(&right.threshold));
    }

    let item_effects = data.get_item_effect(expansion_id, item_id)
      .and_then(|inner_item_effects| Some(inner_item_effects
        .iter().map(|item_effect| data.get_localized_spell_description(expansion_id, language_id, item_effect.spell_id).unwrap()).collect()));

    Ok(ItemTooltip {
      name: data.get_localization(language_id, item.localization_id).unwrap().content,
      icon: data.get_icon(item.icon).unwrap().name,
      quality: item.quality,
      required_level: item.required_level,
      item_level: item.item_level,
      durability: item.max_durability,
      bonding: item.bonding.and_then(|bonding_type| data.get_item_bonding(bonding_type).and_then(|item_bonding| data.get_localization(language_id, item_bonding.localization_id).and_then(|localization| Some(localization.content)))),
      inventory_type: item.inventory_type.and_then(|inventory_type_id| data.get_item_inventory_type(inventory_type_id).and_then(|inventory_type| data.get_localization(language_id, inventory_type.localization_id).and_then(|localization| Some(localization.content)))),
      sheath_type: item.sheath.and_then(|sheath_id| data.get_item_sheath(sheath_id).and_then(|item_sheath| data.get_localization(language_id, item_sheath.localization_id).and_then(|localization| Some(localization.content)))),
      sub_class: data.get_item_class(item.class_id).and_then(|item_class| data.get_localization(language_id, item_class.localization_id).and_then(|localization| Some(localization.content))).unwrap(),
      enchant: None,
      weapon_stat,
      socket,
      item_effects,
      stats,
      armor,
      item_set,
    })
  }

  fn get_character_item(&self, data: &Data, armory: &Armory, language_id: u8, item_id: u32, character_history_id: u32) -> Result<ItemTooltip, TooltipFailure> {
    let character_history_res = armory.get_character_history(character_history_id);
    if character_history_res.is_err() {
      return Err(TooltipFailure::InvalidInput);
    }
    let character_history = character_history_res.unwrap();
    let character = self.get_character(data, armory, character_history.character_id).unwrap();
    let character_gear = character_history.character_info.gear;
    let expansion_id = character.expansion_id;

    let item_tooltip_res = self.get_item(data, language_id, expansion_id, item_id);
    if item_tooltip_res.is_err() {
      return Err(item_tooltip_res.err().unwrap());
    }
    let mut item_tooltip = item_tooltip_res.unwrap();

    // If we have an itemset, check which of these items is active
    if item_tooltip.item_set.is_some() {
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.head);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.neck);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.shoulder);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.back);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.chest);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.shirt);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.tabard);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.wrist);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.glove);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.belt);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.leg);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.boot);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.ring1);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.ring2);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.trinket1);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.trinket2);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.main_hand);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.off_hand);
      check_is_active(item_tooltip.item_set.as_mut().unwrap().set_items.as_mut(), &character_gear.ternary_hand);
    }

    // Apply the enchant
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.head, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.neck, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.shoulder, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.back, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.chest, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.shirt, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.tabard, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.wrist, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.main_hand, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.off_hand, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.ternary_hand, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.glove, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.belt, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.leg, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.boot, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.ring1, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.ring2, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.trinket1, item_id);
    try_apply_enchant(data, expansion_id, language_id, &mut item_tooltip.enchant, &character_gear.trinket2, item_id);

    // Fill sockets
    if item_tooltip.socket.is_some() {
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.head, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.neck, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.shoulder, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.back, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.chest, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.shirt, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.tabard, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.wrist, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.main_hand, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.off_hand, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.ternary_hand, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.glove, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.belt, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.leg, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.boot, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.ring1, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.ring2, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.trinket1, item_id);
      try_fill_socket(data, expansion_id, language_id, item_tooltip.socket.as_mut().unwrap(), &character_gear.trinket2, item_id);
    }

    // Apply the random item property if it exists
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.head, item_id, 0);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.neck, item_id, 2);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.shoulder, item_id, 1);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.back, item_id, 2);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.chest, item_id, 0);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.shirt, item_id, 0);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.tabard, item_id, 0);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.wrist, item_id, 2);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.main_hand, item_id, 3);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.off_hand, item_id, 3);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.ternary_hand, item_id, 4);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.glove, item_id, 1);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.belt, item_id, 1);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.leg, item_id, 0);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.boot, item_id, 1);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.ring1, item_id, 2);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.ring2, item_id, 2);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.trinket1, item_id, 1);
    try_apply_random_item_property(data, expansion_id, language_id, &mut item_tooltip, &character_gear.trinket2, item_id, 1);

    Ok(item_tooltip)
  }
}

fn check_is_active(items_to_check: &mut Vec<SetItem>, item: &Option<CharacterItem>) {
  if item.is_none() {
    return;
  }

  let item_id = item.as_ref().unwrap().item_id;
  for itm in items_to_check {
    if itm.item_id == item_id {
      itm.active = true;
    }
  }
}

fn try_apply_enchant(data: &Data, expansion_id: u8, language_id: u8, enchant: &mut Option<String>, item: &Option<CharacterItem>, item_id: u32) {
  if item.is_none() {
    return;
  }

  let item_res = item.as_ref().unwrap();
  if item_res.item_id != item_id || item_res.enchant_id.is_none() {
    return;
  }
  *enchant = data.get_enchant(expansion_id, item_res.enchant_id.unwrap())
    .and_then(|enchant| data.get_localization(language_id, enchant.localization_id)
      .and_then(|localization| Some(localization.content)));
}

fn try_fill_socket(data: &Data, expansion_id: u8, language_id: u8, socket: &mut Socket, item: &Option<CharacterItem>, item_id: u32) {
  if item.is_none() {
    return;
  }

  let item_res = item.as_ref().unwrap();
  if item_res.item_id != item_id || item_res.enchant_id.is_none() {
    return;
  }

  let socket_slots_length = socket.slots.len();
  for i in 0..item_res.gem_ids.len() {
    let gem_opt = item_res.gem_ids.get(i).unwrap();
    if gem_opt.is_none() {
      continue;
    }

    let gem_item_res = data.get_item(expansion_id, gem_opt.unwrap());
    if gem_item_res.is_none() {
      continue;
    }
    let gem_item = gem_item_res.unwrap();
    let gem = data.get_gem(expansion_id, gem_item.id).unwrap();
    let socket_item = Some(SocketSlotItem {
      icon: data.get_icon(gem_item.icon).unwrap().name,
      effect: data.get_enchant(expansion_id, gem.enchant_id)
        .and_then(|enchant| data.get_localization(language_id, enchant.localization_id)
          .and_then(|localization| Some(localization.content))).unwrap(),
      flag: gem.flag,
    });

    if i < socket_slots_length {
      socket.slots[i].item = socket_item.to_owned();
    } else {
      socket.slots.push(SocketSlot {
        flag: socket_item.as_ref().unwrap().flag,
        item: socket_item.to_owned(),
      });
    }
  }
}

fn try_apply_random_item_property(data: &Data, expansion_id: u8, language_id: u8, item_tooltip: &mut ItemTooltip, item: &Option<CharacterItem>, item_id: u32, suffix_index: u8) {
  if item.is_none() {
    return;
  }

  let item_res = item.as_ref().unwrap();
  if item_res.item_id != item_id || item_res.random_property_id.is_none() {
    return;
  }

  let random_property_id = item_res.random_property_id.as_ref().unwrap();
  let item_random_property = data.get_item_random_property(expansion_id, *random_property_id).unwrap();
  let property_suffix = data.get_localization(language_id, item_random_property.localization_id).unwrap().content;
  let data_item = data.get_item(expansion_id, item_res.item_id).unwrap();
  let property_points = data_item.item_level
    .and_then(|level| data.get_item_random_property_points(expansion_id, level));

  item_tooltip.name += &(" ".to_owned() + &property_suffix);

  for i in 0..item_random_property.enchant_ids.len() {
    let enchant_id = item_random_property.enchant_ids[i];
    let enchant = data.get_enchant(expansion_id, enchant_id).unwrap();
    let effect_value = data.get_localization(language_id, enchant.localization_id).unwrap().content;
    if *random_property_id < 0 {
      let coefficient_value = item_random_property.scaling_coefficients[i];
      let scaling_factor;
      if data_item.quality == 3 {
        scaling_factor = property_points.as_ref().unwrap().good[suffix_index as usize];
      } else if data_item.quality == 4 {
        scaling_factor = property_points.as_ref().unwrap().rare[suffix_index as usize];
      } else if data_item.quality == 5 {
        scaling_factor = property_points.as_ref().unwrap().epic[suffix_index as usize];
      } else {
        scaling_factor = 0;
      }

      let stat_value = ((coefficient_value * scaling_factor as u32) as f64 / 10000.0).round() as u16;
      item_tooltip.stats.as_mut().unwrap().push(Stat {
        value: stat_value.to_owned(),
        name: effect_value.replace("+$i ", "").to_owned()
      });
    } else {
      item_tooltip.stats.as_mut().unwrap().push(Stat {
        value: 0,
        name: effect_value.to_owned()
      });
    }
  }
}