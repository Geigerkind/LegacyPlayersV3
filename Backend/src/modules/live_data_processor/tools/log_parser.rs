use crate::modules::armory::dto::{CharacterDto, CharacterGearDto, CharacterHistoryDto, CharacterInfoDto, CharacterItemDto};
use crate::modules::armory::tools::{GetCharacter, SetCharacter};
use crate::modules::armory::Armory;
use crate::modules::data::tools::{RetrieveEncounter, RetrieveServer};
use crate::modules::data::Data;
use crate::modules::live_data_processor::domain_value::{HitType, School};
use crate::modules::live_data_processor::dto::{AuraApplication, CombatState, DamageComponent, DamageDone, Death, HealDone, InstanceMap, Interrupt, LiveDataProcessorFailure, Message, MessageType, SpellCast, Summon, UnAura, Unit};
use crate::modules::live_data_processor::material::WoWCBTLParser;
use crate::modules::live_data_processor::tools::GUID;
use crate::util::database::{Execute, Select};
use chrono::{Datelike, NaiveDateTime};
use regex::Regex;
use rust_lapper::{Interval, Lapper};
use std::collections::{BTreeSet, HashMap};

pub trait LogParser {
    fn parse(&mut self, db_main: &mut (impl Select + Execute), data: &Data, armory: &Armory, file_content: &str) -> Vec<Message>;
}

impl LogParser for WoWCBTLParser {
    fn parse(&mut self, db_main: &mut (impl Select + Execute), data: &Data, armory: &Armory, file_content: &str) -> Vec<Message> {
        let mut messages = Vec::with_capacity(1000000);
        // TODO: Handle 31/12 => 01/01 raids
        let current_year = NaiveDateTime::from_timestamp((self.start_parse / 1000) as i64, 0).year();
        for line in file_content.split('\n').into_iter() {
            let meta = line.split("  ").collect::<Vec<&str>>();
            if meta.len() != 2 {
                continue;
            }
            if let Ok(timestamp) = NaiveDateTime::parse_from_str(&format!("{}/{}", current_year, meta[0]), "%Y/%m/%d %H:%M:%S%.3f") {
                let message_args = meta[1].trim_end_matches('\r').split(',').collect::<Vec<&str>>();
                let event_timestamp = timestamp.timestamp_millis() as u64;
                if event_timestamp < self.start_parse || event_timestamp > self.end_parse {
                    continue;
                }

                if let Ok(message_types) = parse_log_message(self, data, event_timestamp, message_args) {
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
        // Insert new participants
        let mut remove_unit = BTreeSet::new();
        let mut replace_unit_id = HashMap::new();
        if self.server_id < 0 {
            for (unit_id, (retail_server_id, unit_name_with_server, gear_setups)) in self.retail_player_server.iter() {
                let unit_name_with_server = unit_name_with_server.replace("\"", "");
                let names = unit_name_with_server.split('-').collect::<Vec<&str>>();
                // Create Server or get server
                let mut server = data.get_internal_server_by_retail_id(*retail_server_id);
                if server.is_none() {
                    // TODO: Create patch tag from expansion_id
                    server = Some(data.set_internal_retail_server(db_main, names[1].to_string(), self.expansion_id, "1.13".to_string(), *retail_server_id));
                }
                let server = server.unwrap();
                println!("Found: {:?} => for {} => {}", server, unit_id, unit_name_with_server);
                self.server_id = server.id as i32; // TODO: Better way to determine server id
                if let Some((_, hero_class_id)) = self.found_player.get(&unit_id) {
                    println!("FOUND PLAYER!");
                    // Insert player gear
                    for (_timestamp, gear) in gear_setups.iter() {
                        // TODO: Use timestamp
                        let _ = armory.set_character(
                            db_main,
                            self.server_id as u32,
                            CharacterDto {
                                server_uid: *unit_id,
                                character_history: Some(CharacterHistoryDto {
                                    character_info: CharacterInfoDto {
                                        gear: CharacterGearDto {
                                            head: create_character_item_dto(gear[0].0, gear[0].1),
                                            neck: create_character_item_dto(gear[1].0, gear[1].1),
                                            shoulder: create_character_item_dto(gear[2].0, gear[2].1),
                                            back: create_character_item_dto(gear[14].0, gear[14].1),
                                            chest: create_character_item_dto(gear[4].0, gear[4].1),
                                            shirt: create_character_item_dto(gear[3].0, gear[3].1),
                                            tabard: create_character_item_dto(gear[18].0, gear[18].1),
                                            wrist: create_character_item_dto(gear[8].0, gear[8].1),
                                            main_hand: create_character_item_dto(gear[15].0, gear[15].1),
                                            off_hand: create_character_item_dto(gear[16].0, gear[16].1),
                                            ternary_hand: create_character_item_dto(gear[17].0, gear[17].1),
                                            glove: create_character_item_dto(gear[9].0, gear[9].1),
                                            belt: create_character_item_dto(gear[5].0, gear[5].1),
                                            leg: create_character_item_dto(gear[6].0, gear[6].1),
                                            boot: create_character_item_dto(gear[7].0, gear[7].1),
                                            ring1: create_character_item_dto(gear[10].0, gear[10].1),
                                            ring2: create_character_item_dto(gear[11].0, gear[11].1),
                                            trinket1: create_character_item_dto(gear[12].0, gear[12].1),
                                            trinket2: create_character_item_dto(gear[13].0, gear[13].1),
                                        },
                                        hero_class_id: *hero_class_id,
                                        level: 60, // TODO: FIXME FOR RETAIL TBC
                                        gender: false,
                                        profession1: None,
                                        profession2: None,
                                        talent_specialization: None,
                                        race_id: 1, // TODO: Some unknown race?
                                    },
                                    character_name: names[0].to_string(),
                                    character_guild: None,
                                    character_title: None,
                                    profession_skill_points1: None,
                                    profession_skill_points2: None,
                                    facial: None,
                                    arena_teams: vec![],
                                }),
                            },
                        );
                    }
                }
            }
        } else {
            for (unit_id, (unit_name, hero_class_id)) in self.found_player.iter() {
                let character = armory.get_character_by_name(self.server_id as u32, unit_name.clone());

                if let Some(character) = character {
                    replace_unit_id.insert(*unit_id, character.server_uid);
                } else {
                    // Netherwing and Karazhan provide armory data directly
                    if self.server_id == 4 || self.server_id == 5 {
                        remove_unit.insert(*unit_id);
                        continue;
                    }

                    let _ = armory.set_character(
                        db_main,
                        self.server_id as u32,
                        CharacterDto {
                            server_uid: *unit_id,
                            character_history: Some(CharacterHistoryDto {
                                character_info: CharacterInfoDto {
                                    gear: CharacterGearDto {
                                        head: None,
                                        neck: None,
                                        shoulder: None,
                                        back: None,
                                        chest: None,
                                        shirt: None,
                                        tabard: None,
                                        wrist: None,
                                        main_hand: None,
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
                                    hero_class_id: *hero_class_id,
                                    level: if self.expansion_id == 3 { 80 } else { 70 },
                                    gender: false,
                                    profession1: None,
                                    profession2: None,
                                    talent_specialization: None,
                                    race_id: 1, // TODO: Some unknown race?
                                },
                                character_name: unit_name.clone(),
                                character_guild: None,
                                character_title: None,
                                profession_skill_points1: None,
                                profession_skill_points2: None,
                                facial: None,
                                arena_teams: vec![],
                            }),
                        },
                    );
                }
            }
        }

        let mut current_map = None;
        let mut instance_ids = HashMap::new();
        let mut participants = HashMap::new();
        let mut temp_intervals = Vec::with_capacity(10000);
        self.participation.iter().for_each(|(unit_id, (_, is_player, intervals))| {
            intervals.iter().for_each(|(start, end)| {
                temp_intervals.push(Interval {
                    start: ((*start - 1000) / 1000) as u32,
                    stop: ((*end + 1000) / 1000) as u32, // TODO: Update fixed this, we can use u64
                    val: (*unit_id, *is_player),
                });
            });
        });

        let participants_by_interval = Lapper::new(temp_intervals);
        let mut additional_messages = Vec::with_capacity(20000);
        let mut last_combat_update = HashMap::new();
        for current_message in messages.iter() {
            let current_timestamp = current_message.timestamp;
            let current_message_count = current_message.message_count;

            // Insert Instance Map Messages
            if let Some((map_id, difficulty)) = get_current_map(self, current_timestamp) {
                if !current_map.contains(&(map_id, difficulty)) {
                    current_map = Some((map_id, difficulty));
                    participants.clear();
                };
                let current_ts_in_s = (current_timestamp / 1000) as u32;
                let mut new_participants: HashMap<u64, bool> = HashMap::new();
                for Interval { val: (unit_id, is_player), .. } in participants_by_interval.find(current_ts_in_s - 1, current_ts_in_s + 1).collect::<Vec<&Interval<(u64, bool)>>>().iter() {
                    if !participants.contains_key(unit_id) {
                        new_participants.insert(*unit_id, *is_player);
                    }
                }

                let instance_id = *instance_ids.entry((map_id, difficulty)).or_insert_with(rand::random::<u32>);
                for (unit_id, is_player) in new_participants {
                    // Thaddius is a special snowflake
                    let mut ts_offset: i64 = -1;
                    if !is_player {
                        let entry = unit_id.get_entry();
                        if entry.contains(&15928) || entry.contains(&32535) {
                            ts_offset = -31000;
                        } else if entry.contains(&33651) {
                            ts_offset = -51000;
                        } else if entry.contains(&15990) {
                            ts_offset = -229000;
                        }
                        // Yogg-Saron npcs
                        else if entry.contains(&33966) || entry.contains(&33983) || entry.contains(&33985) || entry.contains(&33988) || entry.contains(&33990) {
                            ts_offset = -56000;
                        }
                    }

                    additional_messages.push(Message {
                        api_version: 0,
                        message_length: 0,
                        timestamp: (current_timestamp as i64 + ts_offset) as u64,
                        message_count: current_message_count,
                        message_type: MessageType::InstanceMap(InstanceMap {
                            map_id: map_id as u32,
                            instance_id,
                            map_difficulty: difficulty.unwrap_or(0),
                            unit: Unit { is_player, unit_id },
                        }),
                    });
                    participants.insert(unit_id, is_player);
                }
            } else if current_map.is_some() {
                for (unit_id, is_player) in participants.iter() {
                    additional_messages.push(Message {
                        api_version: 0,
                        message_length: 0,
                        timestamp: current_timestamp + 1,
                        message_count: current_message_count,
                        message_type: MessageType::InstanceMap(InstanceMap {
                            map_id: 0,
                            instance_id: 0,
                            map_difficulty: 0,
                            unit: Unit { is_player: *is_player, unit_id: *unit_id },
                        }),
                    });
                }
                participants.clear();
                current_map = None;
            }

            // Insert Combat Start/End Events
            // We assume CBT Start when we see it doing sth
            // We assume end if it dies or after a timeout
            match &current_message.message_type {
                MessageType::MeleeDamage(dmg) | MessageType::SpellDamage(dmg) => {
                    add_combat_event(&mut additional_messages, &mut last_combat_update, current_timestamp, current_message_count, &dmg.attacker);
                    add_combat_event(&mut additional_messages, &mut last_combat_update, current_timestamp, current_message_count, &dmg.victim);
                },
                /*
                MessageType::SpellCast(cast) => {
                    add_combat_event(&mut additional_messages, &mut last_combat_update, current_timestamp, current_message_count, &cast.caster);
                    if let Some(target) = &cast.target {
                        add_combat_event(&mut additional_messages, &mut last_combat_update, current_timestamp, current_message_count, target);
                    }
                }
                 */
                MessageType::Death(death) => {
                    additional_messages.push(Message {
                        api_version: 0,
                        message_length: 0,
                        timestamp: current_timestamp + 1,
                        message_count: current_message_count,
                        message_type: MessageType::CombatState(CombatState { unit: death.victim.clone(), in_combat: false }),
                    });
                    last_combat_update.remove(&death.victim.unit_id);

                    if !death.victim.is_player && death.victim.unit_id.is_creature() {
                        let entry_id = death.victim.unit_id.get_entry().unwrap();

                        // If Feugen or Stallag dies, add Thaddius to the combat
                        if entry_id == 15929 || entry_id == 15930 {
                            if let Some(unit_id) = self.participation.iter().find_map(|(unit_id, (_, is_player, _))| {
                                if !*is_player && unit_id.is_creature() && unit_id.get_entry().contains(&15928) {
                                    return Some(unit_id);
                                }
                                None
                            }) {
                                add_combat_event(&mut additional_messages, &mut last_combat_update, current_timestamp, current_message_count, &Unit { is_player: false, unit_id: *unit_id });
                            }
                        }
                    }
                },
                _ => {},
            };
        }

        messages.append(&mut additional_messages);
        if self.server_id == 4 || self.server_id == 5 {
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
        messages
    }
}

fn create_character_item_dto(item_id: u32, enchant_id: Option<u32>) -> Option<CharacterItemDto> {
    if item_id == 0 {
        return None;
    }

    Some(CharacterItemDto {
        item_id,
        random_property_id: None,
        enchant_id,
        gem_ids: vec![],
    })
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

fn add_combat_event(additional_messages: &mut Vec<Message>, last_combat_update: &mut HashMap<u64, u64>, current_timestamp: u64, current_message_count: u64, unit: &Unit) {
    let mut ts_offset: i64 = -1;
    if !unit.is_player {
        let entry = unit.unit_id.get_entry();
        // Wyrmrest Skytalon
        if entry.contains(&32535) {
            ts_offset = -30000;
        // VX-001
        } else if entry.contains(&33651) {
            ts_offset = -50000;
        // Kel'Thuzad
        } else if entry.contains(&15990) {
            ts_offset = -228000;
        }
        // Yogg-Saron npcs
        else if entry.contains(&33966) || entry.contains(&33983) || entry.contains(&33985) || entry.contains(&33988) || entry.contains(&33990) {
            ts_offset = -55000;
            for (unit_id, _) in last_combat_update.clone() {
                if unit_id.get_entry().contains(&33288) {
                    add_combat_event(additional_messages, last_combat_update, current_timestamp, current_message_count, &Unit { is_player: false, unit_id });
                }
            }
        }
    }

    let mut timeout = 60000;
    // Malygos has a bigger timeout
    if !unit.is_player {
        let entry = unit.unit_id.get_entry();
        if entry.contains(&28859) {
            timeout = 180000;
        } else if entry.contains(&15990) {
            timeout = 228000;
        }
    } else {
        timeout = 30000;
    }

    if let Some(last_update) = last_combat_update.get_mut(&unit.unit_id) {
        if current_timestamp - *last_update >= timeout {
            additional_messages.push(Message {
                api_version: 0,
                message_length: 0,
                timestamp: current_timestamp - (current_timestamp - *last_update),
                message_count: current_message_count,
                message_type: MessageType::CombatState(CombatState { unit: unit.clone(), in_combat: false }),
            });
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

fn get_current_map(me: &WoWCBTLParser, current_timestamp: u64) -> Option<(u16, Option<u8>)> {
    let mut map_id = None;
    for (pot_map_id, (_, intervals)) in me.active_map.iter() {
        for (start, end) in intervals {
            if *start <= current_timestamp && *end >= current_timestamp {
                map_id = Some(*pot_map_id);
                break;
            }
        }
    }

    map_id.and_then(|map_id| {
        if me.expansion_id < 3 {
            return Some((map_id, None));
        }

        let mut difficulty_id = None;
        for (pot_diff_id, start, end) in me.active_difficulty.iter() {
            if *start <= current_timestamp && *end >= current_timestamp {
                difficulty_id = Some(*pot_diff_id);
                break;
            }
        }

        if difficulty_id.is_none() {
            if me
                .participation
                .iter()
                .filter(|(_unit_id, (_, is_player, intervals))| intervals.iter().any(|(start, end)| *is_player && *start <= current_timestamp && *end >= current_timestamp))
                .count()
                > 15
            {
                difficulty_id = Some(4);
            } else {
                difficulty_id = Some(3);
            }
        }

        difficulty_id.map(|difficulty_id| (map_id, Some(difficulty_id)))
    })
}

fn parse_log_message(me: &mut WoWCBTLParser, data: &Data, event_timestamp: u64, message_args: Vec<&str>) -> Result<Vec<MessageType>, LiveDataProcessorFailure> {
    Ok(match message_args[0] {
        // TODO: Retail _ABSORBED suffix (https://wow.gamepedia.com/COMBAT_LOG_EVENT)
        "COMBATANT_INFO" => {
            lazy_static! {
                static ref ITEM_REGEX: Regex = Regex::new(r"(\((\d+),(\d+),(\(\d+,\d+,\d+\)|\(\)),\(\),\(\)\))+").unwrap();
            }

            let unit_params = message_args[1].split('-').collect::<Vec<&str>>();
            let unit_id = u64::from_str_radix(unit_params[2], 16).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
            // TODO: Is that hex or dec?
            let server_id = u32::from_str_radix(unit_params[1], 16).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
            let mut gear = Vec::with_capacity(20);
            let character_gear = message_args.join(",");
            for cap in ITEM_REGEX.captures_iter(&character_gear) {
                let item_id = u32::from_str_radix(&cap[2], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
                let enchant_id = if cap.len() > 2 { u32::from_str_radix(&cap[4], 10).ok() } else { None };
                gear.push((item_id, enchant_id));
            }

            if let Some(args) = me.retail_player_server.get_mut(&unit_id) {
                args.2.push((event_timestamp, gear));
            } else {
                me.retail_player_server.insert(unit_id, (server_id, "Unknown".to_string(), vec![(event_timestamp, gear)]));
            }

            // Return error for meta infos
            return Err(LiveDataProcessorFailure::InvalidInput);
        },
        "ENCOUNTER_START" => {
            let retail_encounter_id = u32::from_str_radix(message_args[1], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
            let encounter = data.get_encounter_by_retail_id(retail_encounter_id).expect("I hope I dont forgot to update the table as I add expansions");
            vec![MessageType::EncounterStart(encounter.id)]
        },
        "ENCOUNTER_END" => {
            let retail_encounter_id = u32::from_str_radix(message_args[1], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
            let encounter = data.get_encounter_by_retail_id(retail_encounter_id).expect("I hope I dont forgot to update the table as I add expansions");
            vec![MessageType::EncounterEnd(encounter.id)]
        },
        "SWING_DAMAGE" | "SWING_DAMAGE_LANDED" => {
            if me.server_id < 0 && me.expansion_id == 1 && message_args[0] == "SWING_DAMAGE" {
                return Err(LiveDataProcessorFailure::InvalidInput);
            }

            let attacker = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[1..4])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[1..5])?
            };
            let victim = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[4..7])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[5..9])?
            };
            let (hit_mask, blocked, damage_component) = if me.server_id > 0 { parse_damage(me, &message_args[7..])? } else { parse_damage(me, &message_args[(9 + 17)..])? };
            vec![MessageType::MeleeDamage(DamageDone {
                attacker,
                victim,
                spell_id: None,
                hit_mask,
                blocked,
                damage_over_time: false,
                damage_components: vec![damage_component],
            })]
        },
        "SWING_MISSED" => {
            let attacker = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[1..4])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[1..5])?
            };
            let victim = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[4..7])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[5..9])?
            };
            let (hit_mask, blocked, damage_component) = if me.server_id > 0 { parse_miss(&message_args[7..])? } else { parse_miss(&message_args[9..])? };
            vec![MessageType::MeleeDamage(DamageDone {
                attacker,
                victim,
                spell_id: None,
                hit_mask,
                blocked,
                damage_over_time: false,
                damage_components: damage_component.map(|comp| vec![comp]).unwrap_or_else(Vec::new),
            })]
        },
        "SPELL_DAMAGE" | "SPELL_PERIODIC_DAMAGE" | "RANGE_DAMAGE" | "DAMAGE_SHIELD" | "DAMAGE_SPLIT" => {
            let attacker = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[1..4])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[1..5])?
            };
            let victim = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[4..7])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[5..9])?
            };
            let (spell_id, _school_mask) = if me.server_id > 0 {
                parse_spell_args(me, event_timestamp, &message_args[7..10])?
            } else {
                parse_spell_args(me, event_timestamp, &message_args[9..12])?
            };
            let (hit_mask, blocked, damage_component) = if me.server_id > 0 { parse_damage(me, &message_args[10..])? } else { parse_damage(me, &message_args[(12 + 17)..])? };
            if attacker.is_player {
                me.collect_player(attacker.unit_id, message_args[2], spell_id);
            }
            vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask,
                    blocked,
                    damage_over_time: false, // message_args[0] == "SPELL_PERIODIC_DAMAGE",
                    damage_components: vec![damage_component],
                }),
            ]
        },
        "SPELL_MISSED" | "SPELL_PERIODIC_MISSED" | "RANGE_MISSED" | "DAMAGE_SHIELD_MISSED" => {
            let attacker = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[1..4])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[1..5])?
            };
            let victim = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[4..7])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[5..9])?
            };
            let (spell_id, _school_mask) = if me.server_id > 0 {
                parse_spell_args(me, event_timestamp, &message_args[7..10])?
            } else {
                parse_spell_args(me, event_timestamp, &message_args[9..12])?
            };
            let (hit_mask, blocked, damage_component) = if me.server_id > 0 { parse_miss(&message_args[10..])? } else { parse_miss(&message_args[12..])? };
            if attacker.is_player {
                me.collect_player(attacker.unit_id, message_args[2], spell_id);
            }
            vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask,
                    blocked,
                    damage_over_time: false, // message_args[0] == "SPELL_PERIODIC_MISSED",
                    damage_components: damage_component.map(|comp| vec![comp]).unwrap_or_else(Vec::new),
                }),
            ]
        },
        "SPELL_HEAL" | "SPELL_PERIODIC_HEAL" => {
            let caster = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[1..4])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[1..5])?
            };
            let target = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[4..7])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[5..9])?
            };
            let (spell_id, _school_mask) = if me.server_id > 0 {
                parse_spell_args(me, event_timestamp, &message_args[7..10])?
            } else {
                parse_spell_args(me, event_timestamp, &message_args[9..12])?
            };
            let (amount, overhealing, absorbtion, is_crit) = if me.server_id > 0 { parse_heal(me, &message_args[10..])? } else { parse_heal(me, &message_args[(12 + 17)..])? };
            if caster.is_player {
                me.collect_player(caster.unit_id, message_args[2], spell_id);
            }
            vec![
                MessageType::SpellCast(SpellCast {
                    caster: caster.clone(),
                    target: Some(target.clone()),
                    spell_id,
                    hit_mask: if is_crit { HitType::Crit as u32 } else { HitType::Hit as u32 },
                }),
                MessageType::Heal(HealDone {
                    caster,
                    target,
                    spell_id,
                    total_heal: amount,
                    effective_heal: amount - overhealing,
                    absorb: absorbtion,
                    hit_mask: if is_crit { HitType::Crit as u32 } else { HitType::Hit as u32 },
                }),
            ]
        },
        // TODO: Do we do sth with stacks atm?
        // TODO 2: Retail provides stacks, other versions it needs to be estimated
        // "SPELL_AURA_APPLIED_DOSE" | "SPELL_AURA_REMOVED_DOSE"
        "SPELL_AURA_APPLIED" | "SPELL_AURA_REMOVED" => {
            let caster = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[1..4])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[1..5])?
            };
            let target = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[4..7])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[5..9])?
            };
            let (spell_id, _school_mask) = if me.server_id > 0 {
                parse_spell_args(me, event_timestamp, &message_args[7..10])?
            } else {
                parse_spell_args(me, event_timestamp, &message_args[9..12])?
            };
            if caster.is_player {
                me.collect_player(caster.unit_id, message_args[2], spell_id);
            }
            /*
            let amount = if message_args.len() == 12 {
                i8::from_str_radix(message_args[11], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?
            } else { 0 };
             */
            let is_removed = message_args[0].contains("REMOVED");
            let mut result = vec![MessageType::AuraApplication(AuraApplication {
                caster: caster.clone(),
                target: target.clone(),
                spell_id,
                stack_amount: if is_removed { 0 } else { 1 }, // TODO
                delta: if is_removed { -1 } else { 1 },
            })];

            // Soul Link (Be aware that pet casts onto owner)
            if spell_id == 25228 {
                result.push(MessageType::Summon(Summon { owner: target, unit: caster }));
            }

            result
        },
        "SPELL_CAST_SUCCESS" => {
            let caster = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[1..4])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[1..5])?
            };
            let target = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[4..7])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[5..9])?
            };
            let (spell_id, _school_mask) = if me.server_id > 0 {
                parse_spell_args(me, event_timestamp, &message_args[7..10])?
            } else {
                parse_spell_args(me, event_timestamp, &message_args[9..12])?
            };
            if caster.is_player {
                me.collect_player(caster.unit_id, message_args[2], spell_id);
            }
            let mut result = vec![MessageType::SpellCast(SpellCast {
                caster: caster.clone(),
                target: Some(target.clone()),
                spell_id,
                hit_mask: HitType::Hit as u32,
            })];

            // Mend Pet, Feed Pet, assume summon event
            // Pet Buff food as well
            if spell_id == 48990 || spell_id == 27046 || spell_id == 25228 || spell_id == 43771 || spell_id == 6991 {
                result.push(MessageType::Summon(Summon { owner: caster, unit: target }));
            }

            result
        },
        "SPELL_SUMMON" => {
            let owner = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[1..4])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[1..5])?
            };
            let unit = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[4..7])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[5..9])?
            };
            // TODO: Is that of interest?
            // let (spell_id, _school_mask) = parse_spell_args(m, event_timestamp, &message_args[7..10])?;
            vec![MessageType::Summon(Summon { owner, unit })]
        },
        "UNIT_DIED" | "UNIT_DESTROYED" => {
            let victim = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[4..7])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[5..9])?
            };
            vec![MessageType::Death(Death { cause: None, victim })]
        },
        "SPELL_DISPEL" | "SPELL_AURA_DISPELLED" => {
            let un_aura_caster = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[1..4])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[1..5])?
            };
            let target = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[4..7])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[5..9])?
            };
            let (un_aura_spell_id, _school_mask) = if me.server_id > 0 {
                parse_spell_args(me, event_timestamp, &message_args[7..10])?
            } else {
                parse_spell_args(me, event_timestamp, &message_args[9..12])?
            };
            let (target_spell_id, _target_school_mask) = if me.server_id > 0 {
                parse_spell_args(me, event_timestamp, &message_args[10..13])?
            } else {
                parse_spell_args(me, event_timestamp, &message_args[12..15])?
            };
            if un_aura_caster.is_player {
                me.collect_player(un_aura_caster.unit_id, message_args[2], un_aura_spell_id);
            } else if target.is_player {
                me.collect_player(target.unit_id, message_args[5], target_spell_id);
            }

            vec![
                MessageType::SpellCast(SpellCast {
                    caster: un_aura_caster.clone(),
                    target: Some(target.clone()),
                    spell_id: un_aura_spell_id,
                    hit_mask: HitType::Hit as u32,
                }),
                MessageType::Dispel(UnAura {
                    un_aura_caster,
                    target,
                    aura_caster: None,
                    un_aura_spell_id,
                    target_spell_id,
                    un_aura_amount: 1,
                }),
            ]
        },
        "SPELL_INTERRUPT" => {
            let un_aura_caster = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[1..4])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[1..5])?
            };
            let target = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[4..7])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[5..9])?
            };
            let (un_aura_spell_id, _school_mask) = if me.server_id > 0 {
                parse_spell_args(me, event_timestamp, &message_args[7..10])?
            } else {
                parse_spell_args(me, event_timestamp, &message_args[9..12])?
            };
            let (interrupted_spell_id, _target_school_mask) = if me.server_id > 0 {
                parse_spell_args(me, event_timestamp, &message_args[10..13])?
            } else {
                parse_spell_args(me, event_timestamp, &message_args[12..15])?
            };
            if un_aura_caster.is_player {
                me.collect_player(un_aura_caster.unit_id, message_args[2], un_aura_spell_id);
            } else if target.is_player {
                me.collect_player(target.unit_id, message_args[5], interrupted_spell_id);
            }
            vec![
                MessageType::SpellCast(SpellCast {
                    caster: un_aura_caster,
                    target: Some(target.clone()),
                    spell_id: un_aura_spell_id,
                    hit_mask: HitType::Hit as u32,
                }),
                MessageType::Interrupt(Interrupt { target, interrupted_spell_id }),
            ]
        },
        "SPELL_STOLEN" | "SPELL_AURA_STOLEN" => {
            let un_aura_caster = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[1..4])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[1..5])?
            };
            let target = if me.server_id > 0 {
                parse_unit(me, data, event_timestamp, &message_args[4..7])?
            } else {
                parse_unit(me, data, event_timestamp, &message_args[5..9])?
            };
            let (un_aura_spell_id, _school_mask) = if me.server_id > 0 {
                parse_spell_args(me, event_timestamp, &message_args[7..10])?
            } else {
                parse_spell_args(me, event_timestamp, &message_args[9..12])?
            };
            let (target_spell_id, _target_school_mask) = if me.server_id > 0 {
                parse_spell_args(me, event_timestamp, &message_args[10..13])?
            } else {
                parse_spell_args(me, event_timestamp, &message_args[12..15])?
            };
            if un_aura_caster.is_player {
                me.collect_player(un_aura_caster.unit_id, message_args[2], un_aura_spell_id);
            } else if target.is_player {
                me.collect_player(target.unit_id, message_args[5], target_spell_id);
            }
            vec![
                MessageType::SpellCast(SpellCast {
                    caster: un_aura_caster.clone(),
                    target: Some(target.clone()),
                    spell_id: un_aura_spell_id,
                    hit_mask: HitType::Hit as u32,
                }),
                MessageType::SpellSteal(UnAura {
                    un_aura_caster,
                    target,
                    aura_caster: None,
                    un_aura_spell_id,
                    target_spell_id,
                    un_aura_amount: 1,
                }),
            ]
        },
        // TODO: A lot more events could be parsed and used
        // https://wow.gamepedia.com/index.php?title=COMBAT_LOG_EVENT&oldid=2561876
        _ => return Err(LiveDataProcessorFailure::InvalidInput),
    })
}

fn parse_heal(me: &mut WoWCBTLParser, heal_args: &[&str]) -> Result<(u32, u32, u32, bool), LiveDataProcessorFailure> {
    let amount = u32::from_str_radix(heal_args[0], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    if me.expansion_id == 2 && me.server_id > 0 {
        let critical = heal_args[1].starts_with('1');
        Ok((amount, 0, 0, critical))
    } else {
        let overhealing = u32::from_str_radix(heal_args[1], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
        // TODO: Use as absorbed hint
        let absorbed = u32::from_str_radix(heal_args[2], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
        let critical = heal_args[3].starts_with('1');
        Ok((amount, overhealing, absorbed, critical))
    }
}

fn parse_spell_args(me: &mut WoWCBTLParser, event_timestamp: u64, spell_args: &[&str]) -> Result<(u32, u8), LiveDataProcessorFailure> {
    let spell_id = u32::from_str_radix(spell_args[0], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let school_mask = u8::from_str_radix(spell_args[2].trim_start_matches("0x"), 16).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    if let Some(difficulty_id) = me.get_difficulty_by_spell_id(spell_id) {
        let mut changed = None;
        if let Some((last_difficulty_id, _, last_end_ts)) = me.active_difficulty.last_mut() {
            if *last_difficulty_id == difficulty_id {
                *last_end_ts = event_timestamp;
            } else {
                changed = Some(*last_end_ts);
            }
        } else {
            // We assume the first entry we see is the truth
            me.active_difficulty.push((difficulty_id, 0, event_timestamp));
        }

        if let Some(end_ts) = changed {
            me.active_difficulty.push((difficulty_id, end_ts + 1, event_timestamp));
        }
    }
    Ok((spell_id, school_mask))
}

fn parse_miss(miss_args: &[&str]) -> Result<(u32, u32, Option<DamageComponent>), LiveDataProcessorFailure> {
    let mut extra_hit_mask: u32 = 0;
    let amount_missed = if miss_args.len() == 2 {
        u32::from_str_radix(miss_args[1], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?
    } else if miss_args.len() == 4 {
        // Retail
        if miss_args[1].starts_with('1') {
            extra_hit_mask |= HitType::OffHand as u32;
        }
        if miss_args[3].starts_with('1') {
            extra_hit_mask |= HitType::Crit as u32;
        }
        u32::from_str_radix(miss_args[2], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?
    } else {
        0
    };
    Ok(match miss_args[0] {
        "ABSORB" => (
            HitType::FullAbsorb as u32 | extra_hit_mask,
            0,
            Some(DamageComponent {
                school_mask: School::Physical as u8,
                damage: 0,
                resisted_or_glanced: 0,
                absorbed: amount_missed,
            }),
        ),
        "RESIST" => (
            HitType::FullResist as u32 | extra_hit_mask,
            0,
            Some(DamageComponent {
                school_mask: School::Physical as u8,
                damage: 0,
                resisted_or_glanced: amount_missed,
                absorbed: 0,
            }),
        ),
        "BLOCK" => (HitType::FullBlock as u32 | extra_hit_mask, amount_missed, None),
        "DEFLECT" => (HitType::Deflect as u32 | extra_hit_mask, 0, None),
        "DODGE" => (HitType::Dodge as u32 | extra_hit_mask, 0, None),
        "EVADE" => (HitType::Evade as u32 | extra_hit_mask, 0, None),
        "IMMUNE" => (HitType::Immune as u32 | extra_hit_mask, 0, None),
        "MISS" => (HitType::Miss as u32 | extra_hit_mask, 0, None),
        "PARRY" => (HitType::Parry as u32 | extra_hit_mask, 0, None),
        "REFLECT" => (HitType::Reflect as u32 | extra_hit_mask, 0, None),
        _ => return Err(LiveDataProcessorFailure::InvalidInput),
    })
}

fn parse_damage(me: &mut WoWCBTLParser, damage_args: &[&str]) -> Result<(u32, u32, DamageComponent), LiveDataProcessorFailure> {
    let tbc_shift = if me.expansion_id == 2 && me.server_id > 0 { 1 } else { 0 };

    let amount = u32::from_str_radix(damage_args[0], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let school_mask = u8::from_str_radix(damage_args[2 - tbc_shift], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let resisted = u32::from_str_radix(damage_args[3 - tbc_shift], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let blocked = u32::from_str_radix(damage_args[4 - tbc_shift], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let absorbed = u32::from_str_radix(damage_args[5 - tbc_shift], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let critical = damage_args[6 - tbc_shift].starts_with('1');
    let glancing = damage_args[7 - tbc_shift].starts_with('1');
    let crushing = damage_args[8 - tbc_shift].starts_with('1');
    let is_off_hand = if damage_args.len() == 10 { damage_args[9].starts_with('1') } else { false };
    let mut hit_mask = 0;
    if critical {
        hit_mask |= HitType::Crit as u32;
    } else {
        hit_mask |= HitType::Hit as u32;
    }
    if glancing {
        hit_mask |= HitType::Glancing as u32;
    }
    if crushing {
        hit_mask |= HitType::Crushing as u32;
    }
    if is_off_hand {
        hit_mask |= HitType::OffHand as u32;
    }
    if resisted > 0 {
        hit_mask |= HitType::PartialResist as u32;
    }
    if blocked > 0 {
        hit_mask |= HitType::PartialBlock as u32;
    }
    if absorbed > 0 {
        hit_mask |= HitType::PartialAbsorb as u32;
    }

    Ok((
        hit_mask,
        blocked,
        DamageComponent {
            school_mask,
            damage: amount,
            resisted_or_glanced: resisted,
            absorbed,
        },
    ))
}

fn parse_unit(me: &mut WoWCBTLParser, data: &Data, event_timestamp: u64, unit_args: &[&str]) -> Result<Unit, LiveDataProcessorFailure> {
    let mut unit_id: u64;
    if me.server_id < 0 {
        let unit_params = unit_args[0].split('-').collect::<Vec<&str>>();
        match unit_params[0] {
            "Player" => {
                unit_id = u64::from_str_radix(unit_params[2], 16).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
                if let Some(args) = me.retail_player_server.get_mut(&unit_id) {
                    args.1 = unit_args[1].to_string();
                } else {
                    // TODO: Is that in hex or decimal ?!
                    let server_id = u32::from_str_radix(unit_params[1], 16).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
                    me.retail_player_server.insert(unit_id, (server_id, unit_args[1].to_string(), Vec::new()));
                }
            },
            // Ignore GameObject and Vignette
            "Pet" | "Vehicle" => {
                let spawn_uid = u64::from_str_radix(unit_params[6], 16).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
                let npc_id = u64::from_str_radix(unit_params[5], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
                unit_id = 0;
                unit_id |= 0xF140000000000000;
                unit_id |= npc_id.rotate_left(24);
                unit_id |= spawn_uid;
            },
            "Creature" => {
                let spawn_uid = u64::from_str_radix(unit_params[6], 16).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
                let npc_id = u64::from_str_radix(unit_params[5], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
                unit_id = 0;
                unit_id |= 0xF130000000000000;
                unit_id |= npc_id.rotate_left(24);
                unit_id |= spawn_uid;
            },
            _ => return Err(LiveDataProcessorFailure::InvalidInput),
        };
    } else {
        // let is_player = (u32::from_str_radix(unit_args[2].trim_start_matches("0x"), 16).map_err(|_| LiveDataProcessorFailure::InvalidInput)? & 0x400) != 0;
        unit_id = u64::from_str_radix(unit_args[0].trim_start_matches("0x"), 16).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
        if me.server_id == 6 && unit_id.get_high() & 0x0FF0 == 0x0110 {
            unit_id &= 0x0000000000FFFFFF;
        }

        // Each non npc pet gets the id 0xFFFF (Has flags 0xF140)
        if unit_id.is_pet() {
            let mut new_unit_id = unit_id;
            new_unit_id = (new_unit_id & 0x000000FFFF000000).rotate_right(24);
            new_unit_id |= 0x000000FFFF000000;
            new_unit_id |= 0xF140000000000000;
            unit_id = new_unit_id;
        }
    }

    let is_player = unit_id.is_player();
    if !is_player {
        if let Some(npc_id) = unit_id.get_entry() {
            if let Some(map_id) = me.get_active_map(data, npc_id) {
                let (last_ts, activity_intervals) = me.active_map.entry(map_id).or_insert((event_timestamp, vec![(event_timestamp, event_timestamp)]));
                let last_entry = activity_intervals.last_mut().unwrap();
                if (event_timestamp - last_entry.1) <= 30000 {
                    last_entry.1 = event_timestamp;
                } else {
                    activity_intervals.push((event_timestamp, event_timestamp));
                }
                *last_ts = event_timestamp;
            }
        }
    }

    let (last_ts, _is_player, activity_intervals) = me.participation.entry(unit_id).or_insert((event_timestamp, is_player, vec![(event_timestamp, event_timestamp)]));
    let last_entry = activity_intervals.last_mut().unwrap();
    if (event_timestamp - last_entry.1) <= 60000 {
        last_entry.1 = event_timestamp;
    } else {
        activity_intervals.push((event_timestamp, event_timestamp));
    }
    *last_ts = event_timestamp;

    Ok(Unit { is_player, unit_id })
}
