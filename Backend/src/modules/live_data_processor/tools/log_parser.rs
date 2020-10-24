use crate::modules::armory::tools::{GetCharacter, SetCharacter};
use crate::modules::armory::Armory;
use crate::modules::data::tools::{RetrieveNPC, RetrieveServer};
use crate::modules::data::Data;
use crate::modules::live_data_processor::dto::{CombatState, InstanceMap, Message, MessageType, Unit};
use crate::modules::live_data_processor::material::{RetrieveActiveMap, Participant};
use crate::modules::live_data_processor::tools::cbl_parser::CombatLogParser;
use crate::modules::live_data_processor::tools::GUID;
use crate::util::database::{Execute, Select};
use chrono::{Datelike, NaiveDateTime};
use rust_lapper::{Interval, Lapper};
use std::collections::{BTreeSet, HashMap};

pub fn parse_cbl(parser: &mut impl CombatLogParser, db_main: &mut (impl Select + Execute), data: &Data, armory: &Armory, file_content: &str, start_parse: u64, end_parse: u64) -> Option<(u32, Vec<Message>)> {
    let mut messages = Vec::with_capacity(1000000);

    // Pre processing
    // TODO: Handle 31/12 => 01/01 raids
    let current_year = NaiveDateTime::from_timestamp((start_parse / 1000) as i64, 0).year();
    for line in file_content.split('\n').into_iter() {
        let meta = line.split("  ").collect::<Vec<&str>>();
        if meta.len() != 2 {
            continue;
        }
        if let Ok(timestamp) = NaiveDateTime::parse_from_str(&format!("{}/{}", current_year, meta[0]), "%Y/%m/%d %H:%M:%S%.3f") {
            let event_timestamp = timestamp.timestamp_millis() as u64;
            if event_timestamp < start_parse || event_timestamp > end_parse {
                continue;
            }

            if let Some(message_types) = parser.parse_cbl_line(data, event_timestamp, meta[1].trim_end_matches("\r")) {
                let mut message_count = (messages.len() + message_types.len()) as u64;
                for message_type in message_types {
                    message_count -= 1;
                    messages.push(Message {
                        api_version: 0,
                        message_length: 0,
                        timestamp: event_timestamp,
                        message_count,
                        message_type,
                    });
                }
            }
        }
    }

    // Post processing step
    parser.do_message_post_processing(data, &mut messages);

    let expansion_id = parser.get_expansion_id();
    let mut server_id = parser.get_server_id();
    if let Some(involved_server) = parser.get_involved_server() {
        for (retail_server_id, server_name, patch_tag) in involved_server {
            let mut server = data.get_internal_server_by_retail_id(retail_server_id);
            if server.is_none() {
                server = Some(data.set_internal_retail_server(db_main, server_name, expansion_id, patch_tag, retail_server_id));
            }
            let server = server.unwrap();
            server_id = Some(server.id); // TODO: Better way to determine server id
        }
    }

    let server_id = server_id?;

    let mut remove_unit = BTreeSet::new();
    let mut replace_unit_id = HashMap::new();
    for (retail_server_id, character_dto) in parser.get_involved_character_builds() {
        let server_id = retail_server_id.map(|id| data.get_internal_server_by_retail_id(id).unwrap().id).unwrap_or(server_id);
        if server_id == 4 || server_id == 5 {
            if let Some(character) = armory.get_character_by_name(server_id, character_dto.character_history.as_ref().unwrap().character_name.clone()) {
                replace_unit_id.insert(character_dto.server_uid, character.server_uid);
            } else {
                remove_unit.insert(character_dto.server_uid);
                continue;
            }
        } else {
            // Only set char if gear is not empty or char does not exist
            if let Some(character_info) = &character_dto.character_history {
                if character_info.character_info.gear.is_naked() {
                    if let Some(character) = armory.get_character_by_uid(server_id, character_dto.server_uid) {
                        if let Some(last_update) = character.last_update {
                            if last_update.character_info.hero_class_id != 12 {
                                continue;
                            }
                        }
                    }
                }
            }
            let _ = armory.set_character(db_main, server_id, character_dto);
        }
    }

    // Pre-fill instance ids
    let mut instance_ids = HashMap::new();
    for Message { message_type, .. } in messages.iter() {
        if let MessageType::InstanceMap(map) = message_type {
            // TODO (If I ever get more of these events): This only works for vanilla!
            instance_ids.insert((map.map_id as u16, None), map.instance_id);
        }
    }

    let mut temp_intervals = Vec::with_capacity(4000);
    let mut player_temp_intervals = Vec::with_capacity(1000);

    let mut parsed_participants = parser.get_participants();
    let mut incombat_participant_helper = Participant::new(0xF13000FFFE000000, false, "CBT Helper".to_string(), 10000);
    incombat_participant_helper.active_intervals.push((10001, u64::MAX.rotate_right(2)));
    parsed_participants.iter().for_each(|participant| {
        participant.active_intervals.iter().for_each(|(start, end)| {
            temp_intervals.push(Interval {
                start: *start - 200,
                stop: *end + 200,
                val: (participant.id, participant.is_player),
            });

            temp_intervals.push(Interval {
                start: *start - 200,
                stop: *end + 200,
                val: (incombat_participant_helper.id, incombat_participant_helper.is_player),
            });

            if participant.is_player {
                player_temp_intervals.push(Interval {
                    start: *start - 200,
                    stop: *end + 200,
                    val: participant.id,
                });
            }
        });
    });
    parsed_participants.push(incombat_participant_helper);

    let participants_by_interval = Lapper::new(temp_intervals);
    let player_participants_by_interval = Lapper::new(player_temp_intervals);
    let active_maps = parser.get_active_maps();

    let mut current_map = None;
    let mut participants = HashMap::new();
    let mut last_combat_update = HashMap::new();
    let mut additional_messages = Vec::with_capacity(20000);
    for Message { timestamp, message_count, message_type, .. } in messages.iter() {
        // Insert Instance Map Messages
        if let Some((map_id, difficulty)) = active_maps.get_current_active_map(&player_participants_by_interval, expansion_id, *timestamp) {
            if !current_map.contains(&(map_id, difficulty)) {
                current_map = Some((map_id, difficulty));
                participants.clear();
            };

            let mut new_participants: HashMap<u64, bool> = HashMap::new();
            // TODO: This lapper does not seem to find all participants within that interval, it fails for large intervals
            for Interval { val: (unit_id, is_player), .. } in participants_by_interval.find(*timestamp - 100, *timestamp + 100).filter(|Interval { val: (unit_id, _), .. }| !participants.contains_key(unit_id)) {
                new_participants.insert(*unit_id, *is_player);
            }

            let instance_id = *instance_ids.entry((map_id, difficulty)).or_insert_with(rand::random::<u32>);
            for (unit_id, is_player) in new_participants {
                let mut ts_offset: i64 = -1;
                if !is_player {
                    if let Some(entry) = unit_id.get_entry() {
                        if let Some(delay) = parser.get_npc_appearance_offset(entry) {
                            ts_offset = delay - 100;
                        }
                    }
                }

                additional_messages.push(Message::new_parsed(
                    (*timestamp as i64 + ts_offset) as u64,
                    *message_count,
                    MessageType::InstanceMap(InstanceMap {
                        map_id: map_id as u32,
                        instance_id,
                        map_difficulty: difficulty.unwrap_or(0),
                        unit: Unit { is_player, unit_id },
                    }),
                ));
                participants.insert(unit_id, is_player);
            }
        } else if current_map.is_some() {
            for (unit_id, is_player) in participants.iter() {
                additional_messages.push(Message::new_parsed(
                    *timestamp + 1,
                    *message_count,
                    MessageType::InstanceMap(InstanceMap {
                        map_id: 0,
                        instance_id: 0,
                        map_difficulty: 0,
                        unit: Unit { is_player: *is_player, unit_id: *unit_id },
                    }),
                ));
            }
            participants.clear();
            current_map = None;
        }

        // Insert Combat Start/End Events
        // We assume CBT Start when we see it doing sth
        // We assume end if it dies or after a timeout
        match &message_type {
            MessageType::MeleeDamage(dmg) | MessageType::SpellDamage(dmg) => {
                add_combat_event(parser, data, expansion_id, &mut additional_messages, &mut last_combat_update, *timestamp, *message_count, &dmg.attacker);
                add_combat_event(parser, data, expansion_id, &mut additional_messages, &mut last_combat_update, *timestamp, *message_count, &dmg.victim);
            },
            MessageType::Death(death) => {
                if !death.victim.is_player {
                    if let Some(entry) = death.victim.unit_id.get_entry() {
                        if let Some(implied_npc_ids) = parser.get_death_implied_npc_combat_state_and_offset(entry) {
                            for (npc_id, delay_ts) in implied_npc_ids {
                                if let Some(unit_id) = parsed_participants.iter().find_map(|participant| {
                                    if !participant.is_player && participant.id.get_entry().contains(&npc_id) {
                                        return Some(participant.id);
                                    }
                                    None
                                }) {
                                    add_combat_event(parser, data, expansion_id, &mut additional_messages, &mut last_combat_update, (*timestamp as i64 + delay_ts) as u64, *message_count - 1, &Unit { is_player: false, unit_id });
                                }
                            }
                        }
                    }
                }

                additional_messages.push(Message {
                    api_version: 0,
                    message_length: 0,
                    timestamp: *timestamp + 1,
                    message_count: *message_count,
                    message_type: MessageType::CombatState(CombatState { unit: death.victim.clone(), in_combat: false }),
                });
                last_combat_update.remove(&death.victim.unit_id);
            },
            _ => {},
        };
    }

    if let Some(mut bonus_msgs) = parser.get_bonus_messages() {
        messages.append(&mut bonus_msgs);
    }
    messages.append(&mut additional_messages);
    if server_id == 4 || server_id == 5 {
        messages = messages
            .into_iter()
            .filter(|msg| !is_in_remove_list(&remove_unit, &msg.message_type))
            .map(|mut msg| {
                replace_ids(&replace_unit_id, &mut msg.message_type);
                msg
            })
            .collect();
    }
    messages.sort_by(|left, right| left.timestamp.cmp(&right.timestamp));
    Some((server_id, messages))
}

fn replace_ids(replace_unit_id: &HashMap<u64, u64>, message_type: &mut MessageType) {
    match message_type {
        MessageType::SpellDamage(dmg) | MessageType::MeleeDamage(dmg) => {
            replace_id(replace_unit_id, &mut dmg.attacker);
            replace_id(replace_unit_id, &mut dmg.victim);
        },
        MessageType::Heal(heal) => {
            replace_id(replace_unit_id, &mut heal.caster);
            replace_id(replace_unit_id, &mut heal.target);
        },
        MessageType::Death(death) => {
            replace_id(replace_unit_id, &mut death.victim);
        },
        MessageType::AuraApplication(aura) => {
            replace_id(replace_unit_id, &mut aura.caster);
            replace_id(replace_unit_id, &mut aura.target);
        },
        // Aura Cast always None
        MessageType::SpellSteal(un_aura) | MessageType::Dispel(un_aura) => {
            replace_id(replace_unit_id, &mut un_aura.un_aura_caster);
            replace_id(replace_unit_id, &mut un_aura.target);
        },
        MessageType::Interrupt(interrupt) => {
            replace_id(replace_unit_id, &mut interrupt.target);
        },
        MessageType::SpellCast(cast) => {
            replace_id(replace_unit_id, &mut cast.caster);
            if let Some(target) = &mut cast.target {
                replace_id(replace_unit_id, target);
            }
        },
        MessageType::Summon(summon) => {
            replace_id(replace_unit_id, &mut summon.unit);
            replace_id(replace_unit_id, &mut summon.owner);
        },
        MessageType::CombatState(cbt) => {
            replace_id(replace_unit_id, &mut cbt.unit);
        },
        MessageType::InstanceMap(map) => {
            replace_id(replace_unit_id, &mut map.unit);
        },
        _ => {},
    };
}

fn replace_id(replace_unit_id: &HashMap<u64, u64>, unit: &mut Unit) {
    if unit.is_player {
        if let Some(char_id) = replace_unit_id.get(&unit.unit_id) {
            unit.unit_id = *char_id;
        } else {
            unit.unit_id = 0;
        }
    }
}

fn is_in_remove_list(remove_unit: &BTreeSet<u64>, message_type: &MessageType) -> bool {
    match &message_type {
        MessageType::SpellDamage(dmg) | MessageType::MeleeDamage(dmg) => remove_unit.contains(&dmg.attacker.unit_id) || remove_unit.contains(&dmg.victim.unit_id),
        MessageType::Heal(heal) => remove_unit.contains(&heal.caster.unit_id) || remove_unit.contains(&heal.target.unit_id),
        MessageType::Death(death) => remove_unit.contains(&death.victim.unit_id),
        MessageType::AuraApplication(aura) => remove_unit.contains(&aura.caster.unit_id) || remove_unit.contains(&aura.target.unit_id),
        // Aura Cast always None
        MessageType::SpellSteal(un_aura) | MessageType::Dispel(un_aura) => remove_unit.contains(&un_aura.un_aura_caster.unit_id) || remove_unit.contains(&un_aura.target.unit_id),
        MessageType::Interrupt(interrupt) => remove_unit.contains(&interrupt.target.unit_id),
        MessageType::SpellCast(cast) => remove_unit.contains(&cast.caster.unit_id) || (cast.target.is_some() && remove_unit.contains(&cast.target.as_ref().unwrap().unit_id)),
        MessageType::Summon(summon) => remove_unit.contains(&summon.unit.unit_id) || remove_unit.contains(&summon.owner.unit_id),
        MessageType::CombatState(cbt) => remove_unit.contains(&cbt.unit.unit_id),
        MessageType::InstanceMap(map) => remove_unit.contains(&map.unit.unit_id),
        _ => false,
    }
}

fn add_combat_event(parser: &impl CombatLogParser, data: &Data, expansion_id: u8, additional_messages: &mut Vec<Message>, last_combat_update: &mut HashMap<u64, u64>, current_timestamp: u64, current_message_count: u64, unit: &Unit) {
    let ts_offset: i64 = -1;
    let mut timeout = 60000;
    let mut current_unit_is_boss = false;
    if let Some(entry) = unit.unit_id.get_entry() {
        /*
        if let Some(delay) = parser.get_npc_appearance_offset(entry) {
            ts_offset = delay;
        }
         */

        if let Some(implied_in_combat_npc_ids) = parser.get_in_combat_implied_npc_combat(entry) {
            for (unit_id, _) in last_combat_update.clone() {
                for npc_id in &implied_in_combat_npc_ids {
                    if unit_id.get_entry().contains(npc_id) {
                        add_combat_event(parser, data, expansion_id, additional_messages, last_combat_update, current_timestamp, current_message_count, &Unit { is_player: false, unit_id });
                    }
                }
            }
        }

        if let Some(delay) = parser.get_npc_timeout(entry) {
            timeout = delay;
        }

        // For bosses consider timeout of same entry not unit id
        current_unit_is_boss = data.get_npc(expansion_id, entry).map(|npc| npc.is_boss).contains(&true);
        if current_unit_is_boss {
            for (unit_id, last_update) in last_combat_update.clone() {
                if unit_id.get_entry().contains(&entry) && current_timestamp - last_update >= timeout {
                    additional_messages.push(Message {
                        api_version: 0,
                        message_length: 0,
                        timestamp: current_timestamp - (current_timestamp - last_update),
                        message_count: current_message_count,
                        message_type: MessageType::CombatState(CombatState {
                            unit: Unit { is_player: false, unit_id },
                            in_combat: false,
                        }),
                    });
                    last_combat_update.remove(&unit_id);
                }
            }
        }
    } else if !unit.is_player {
        timeout = 30000;
    }

    if let Some(last_update) = last_combat_update.get_mut(&unit.unit_id) {
        if current_timestamp - *last_update >= timeout {
            if !current_unit_is_boss {
                additional_messages.push(Message {
                    api_version: 0,
                    message_length: 0,
                    timestamp: current_timestamp - (current_timestamp - *last_update),
                    message_count: current_message_count,
                    message_type: MessageType::CombatState(CombatState { unit: unit.clone(), in_combat: false }),
                });
            }
            additional_messages.push(Message {
                api_version: 0,
                message_length: 0,
                timestamp: (current_timestamp as i64 + ts_offset) as u64,
                message_count: current_message_count,
                message_type: MessageType::CombatState(CombatState { unit: unit.clone(), in_combat: true }),
            });
        }
        *last_update = current_timestamp;
    } else {
        additional_messages.push(Message {
            api_version: 0,
            message_length: 0,
            timestamp: (current_timestamp as i64 + ts_offset) as u64,
            message_count: current_message_count,
            message_type: MessageType::CombatState(CombatState { unit: unit.clone(), in_combat: true }),
        });
        last_combat_update.insert(unit.unit_id, current_timestamp);
    }
}
