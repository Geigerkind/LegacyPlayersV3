use crate::dto::Failure;
use crate::modules::data::Data;
use crate::modules::data::tools::{RetrieveEnchant, RetrieveIcon, RetrieveItem, RetrieveItemBonding, RetrieveItemClass, RetrieveItemDamage, RetrieveItemDamageType, RetrieveItemEffect, RetrieveItemInventoryType, RetrieveItemsetEffect, RetrieveItemsetName, RetrieveItemSheath, RetrieveItemSocket, RetrieveItemStat, RetrieveLocalization, RetrieveStatType, SpellDescription};
use crate::modules::tooltip::domain_value::{ItemSet, ItemTooltip, SetEffect, SetItem, Socket, SocketSlot, Stat, WeaponDamage, WeaponStat};
use crate::modules::tooltip::Tooltip;

pub trait RetrieveItemTooltip {
  fn get_item(&self, data: &Data, language_id: u8, expansion_id: u8, item_id: u32) -> Result<ItemTooltip, Failure>;
  fn get_character_item(&self, data: &Data, language_id: u8, expansion_id: u8, item_id: u32, character_history_id: u32) -> Result<ItemTooltip, Failure>;
}

impl RetrieveItemTooltip for Tooltip {
  fn get_item(&self, data: &Data, language_id: u8, expansion_id: u8, item_id: u32) -> Result<ItemTooltip, Failure> {
    let item_res = data.get_item(expansion_id, item_id);
    if item_res.is_none() {
      return Err(Failure::InvalidInput);
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

  fn get_character_item(&self, data: &Data, language_id: u8, expansion_id: u8, item_id: u32, character_history_id: u32) -> Result<ItemTooltip, Failure> {
    unimplemented!()
  }
}