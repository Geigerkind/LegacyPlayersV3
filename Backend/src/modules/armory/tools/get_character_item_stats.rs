use crate::modules::armory::domain_value::{CharacterGear, CharacterItem};
use crate::modules::armory::dto::CharacterStat;
use crate::modules::data::Stat;
use crate::modules::data::Data;
use crate::modules::data::tools::{RetrieveItem, RetrieveItemsetName, RetrieveItemsetEffect, SpellDescription, RetrieveLocalization, RetrieveStatType, RetrieveItemStat, RetrieveItemEffect, RetrieveEnchant, RetrieveItemSocket, RetrieveGem};

pub fn get_character_stats(data: &Data, language_id: u8, expansion_id: u8, gear: &CharacterGear) -> Vec<CharacterStat> {
    let mut acc = get_item_stats(data, expansion_id, &gear.head);
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.neck));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.shoulder));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.back));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.chest));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.tabard));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.shirt));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.wrist));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.main_hand));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.off_hand));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.ternary_hand));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.glove));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.belt));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.leg));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.boot));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.ring1));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.ring2));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.trinket1));
    merge_character_stat_vec(&mut acc, get_item_stats(data, expansion_id, &gear.trinket2));

    // Stats from set bonus
    let gear_to_vec_item_ids = gear_to_item_id_vec(&gear);
    let mut considered_set_item_ids: Vec<u32> = Vec::new();
    for item_id in gear_to_vec_item_ids {
        if considered_set_item_ids.iter().find(|&&id| id == item_id).is_some() {
            continue;
        }
        if let Some(item) = data.get_item(expansion_id, item_id) {
            if let Some(itemset_id) = item.itemset {
                let mut itemset_item_ids = data.get_itemset_item_ids(expansion_id, itemset_id).unwrap();
                considered_set_item_ids.append(&mut itemset_item_ids);
                data.get_itemset_effects(expansion_id, itemset_id).unwrap().iter()
                    .for_each(|item_effect|
                        merge_character_stat_vec(&mut acc, data.parse_stats(expansion_id, item_effect.spell_id)))
            }
        }
    }

    acc.sort_by(|left, right| left.stat_type.cmp(&right.stat_type));
    acc.iter().map(|stat| CharacterStat {
        stat_type: data.get_localization(language_id, data.get_stat_type(stat.stat_type).unwrap().localization_id).unwrap().content.to_owned(),
        stat_value: stat.stat_value,
    }).collect()
}

fn get_item_stats(data: &Data, expansion_id: u8, item: &Option<CharacterItem>) -> Vec<Stat> {
    let mut stats = Vec::new();
    if item.is_none() {
        return stats;
    }
    let item = item.as_ref().unwrap();

    // Item stats from the template
    if let Some(item_stats) = data.get_item_stats(expansion_id, item.item_id) {
        item_stats.iter().for_each(|item_stat| {
            stats.push(item_stat.stat.to_owned());
        });
    }

    if let Some(item_effect_vec) = data.get_item_effect(expansion_id, item.item_id) {
        item_effect_vec.iter().for_each(|item_effect|
            merge_character_stat_vec(&mut stats, data.parse_stats(expansion_id, item_effect.spell_id)));
    }

    // Stats from enchantments
    if let Some(enchant_id) = item.enchant_id {
        let enchant = data.get_enchant(expansion_id, enchant_id).unwrap();
        merge_character_stat_vec(&mut stats, enchant.stats);
    }

    // Stats from gems
    let mut socket_bonus = true;
    let item_socket = data.get_item_socket(expansion_id, item.item_id);
    for i in 0..item.gem_ids.len() {
        if let Some(gem_id) = item.gem_ids[i] {
            let gem_item = data.get_gem(expansion_id, gem_id).unwrap();
            let enchant = data.get_enchant(expansion_id, gem_item.enchant_id).unwrap();
            merge_character_stat_vec(&mut stats, enchant.stats);
            if let Some(socket_info) = item_socket.as_ref() {
                if i < socket_info.slots.len() {
                    socket_bonus = socket_bonus && socket_info.slots[i] == socket_info.slots[i] & gem_item.flag;
                }
            }
        } else {
            if let Some(socket_info) = item_socket.as_ref() {
                if i < socket_info.slots.len() {
                    socket_bonus = false;
                }
            }
        }
    }
    if let Some(socket_info) = item_socket.as_ref() {
        if socket_bonus {
            let enchant = data.get_enchant(expansion_id, socket_info.bonus).unwrap();
            merge_character_stat_vec(&mut stats, enchant.stats);
        }
    }

    stats
}

fn merge_character_stat_vec(acc: &mut Vec<Stat>, input: Vec<Stat>) {
    for stat in input {
        let acc_stat = acc.iter_mut().find(|inner_stat| inner_stat.stat_type == stat.stat_type);
        if acc_stat.is_some() {
            acc_stat.unwrap().stat_value += stat.stat_value;
        } else {
            acc.push(stat);
        }
    }
}

fn gear_to_item_id_vec(gear: &CharacterGear) -> Vec<u32> {
    let mut item_ids = Vec::new();
    if let Some(item) = &gear.head { item_ids.push(item.item_id) }
    if let Some(item) = &gear.neck { item_ids.push(item.item_id) }
    if let Some(item) = &gear.shoulder { item_ids.push(item.item_id) }
    if let Some(item) = &gear.back { item_ids.push(item.item_id) }
    if let Some(item) = &gear.chest { item_ids.push(item.item_id) }
    if let Some(item) = &gear.tabard { item_ids.push(item.item_id) }
    if let Some(item) = &gear.shirt { item_ids.push(item.item_id) }
    if let Some(item) = &gear.wrist { item_ids.push(item.item_id) }
    if let Some(item) = &gear.main_hand { item_ids.push(item.item_id) }
    if let Some(item) = &gear.off_hand { item_ids.push(item.item_id) }
    if let Some(item) = &gear.ternary_hand { item_ids.push(item.item_id) }
    if let Some(item) = &gear.glove { item_ids.push(item.item_id) }
    if let Some(item) = &gear.belt { item_ids.push(item.item_id) }
    if let Some(item) = &gear.leg { item_ids.push(item.item_id) }
    if let Some(item) = &gear.boot { item_ids.push(item.item_id) }
    if let Some(item) = &gear.ring1 { item_ids.push(item.item_id) }
    if let Some(item) = &gear.ring2 { item_ids.push(item.item_id) }
    if let Some(item) = &gear.trinket1 { item_ids.push(item.item_id) }
    if let Some(item) = &gear.trinket2 { item_ids.push(item.item_id) }
    item_ids
}