use crate::modules::data::Data;
use crate::modules::live_data_processor::dto::{Unit, Message, MessageType, Summon, InstanceMap, Loot};
use crate::modules::live_data_processor::material::{ActiveMapMap, CollectActiveMap, Participant};
use std::collections::{BTreeSet, HashMap};
use chrono::NaiveDateTime;
use crate::modules::live_data_processor::tools::GUID;
use crate::modules::armory::tools::strip_talent_specialization;
use time_util::now;
use regex::Regex;

pub struct WoWTBCParser {
    pub server_id: u32,

    pub participants: HashMap<u64, Participant>,
    pub active_map: ActiveMapMap,

    // Hacky
    pub bonus_messages: Vec<Message>,
}

impl WoWTBCParser {
    pub fn new(server_id: u32, armory_content: Option<String>) -> Self {
        let mut parser = WoWTBCParser {
            server_id,
            participants: Default::default(),
            active_map: Default::default(),
            bonus_messages: Default::default(),
        };
        if let Some(armory_content) = armory_content {
            parser.parse_armory(armory_content);
        }
        parser
    }

    pub fn collect_participant(&mut self, unit: &Unit, name: &str, now: u64) {
        if let Some(participants) = self.participants.get_mut(&unit.unit_id) {
            participants.add_participation_point(now);
        } else {
            self.participants.insert(unit.unit_id, Participant::new(unit.unit_id, unit.is_player, name.replace("\"", ""), now));
        }
    }

    pub fn collect_participant_class(&mut self, unit: &Unit, spell_id: u32) {
        lazy_static! {
            static ref WARRIOR_SPELLS: BTreeSet<u32> = [30335, 30339, 30340, 20647, 29707, 12328, 12723, 44949, 1680, 12721, 34428, 30330, 25242, 30356, 25212, 30022, 30357].iter().cloned().collect();
            static ref PALADIN_SPELLS: BTreeSet<u32> = [27173, 27179, 1042, 10328, 27136, 27155, 27156, 27164, 27162, 27163, 20355, 27157, 27137, 27159, 35395, 31892, 32221, 31893, 31898, 32220]
                .iter()
                .cloned()
                .collect();
            static ref ROGUE_SPELLS: BTreeSet<u32> = [26862, 13877, 22482, 27187, 26867, 26865, 26884].iter().cloned().collect();
            static ref PRIEST_SPELLS: BTreeSet<u32> = [25387, 25375, 25368, 34917, 25213, 10963, 25235].iter().cloned().collect();
            static ref HUNTER_SPELLS: BTreeSet<u32> = [34120, 75, 27021, 27019, 27026].iter().cloned().collect();
            static ref MAGE_SPELLS: BTreeSet<u32> = [30451, 36032, 27072, 27087, 27070, 27082, 33043, 12654, 27074, 27079, 25028, 27086].iter().cloned().collect();
            static ref WARLOCK_SPELLS: BTreeSet<u32> = [27209, 27243, 27285, 30910, 27215, 30546, 29341, 27218, 27223, 30405, 30911].iter().cloned().collect();
            static ref SHAMAN_SPELLS: BTreeSet<u32> = [25449, 37661, 45296, 25442, 45302, 25537, 25457, 25454, 25504, 33750, 32175, 32176, 17364, 10622, 25422, 25423, 25357, 25396, 331, 25420]
                .iter()
                .cloned()
                .collect();
            static ref DRUID_SPELLS: BTreeSet<u32> = [26986, 26988, 27013, 27002, 27008, 33983, 33987, 33763, 33778, 26982, 26980].iter().cloned().collect();
        }

        if !unit.is_player {
            return;
        }

        if let Some(participant) = self.participants.get_mut(&unit.unit_id) {
            if participant.hero_class_id.is_none() {
                if WARRIOR_SPELLS.contains(&spell_id) {
                    participant.hero_class_id = Some(1);
                } else if PALADIN_SPELLS.contains(&spell_id) {
                    participant.hero_class_id = Some(2);
                } else if HUNTER_SPELLS.contains(&spell_id) {
                    participant.hero_class_id = Some(3);
                } else if ROGUE_SPELLS.contains(&spell_id) {
                    participant.hero_class_id = Some(4);
                } else if PRIEST_SPELLS.contains(&spell_id) {
                    participant.hero_class_id = Some(5);
                } else if SHAMAN_SPELLS.contains(&spell_id) {
                    participant.hero_class_id = Some(7);
                } else if MAGE_SPELLS.contains(&spell_id) {
                    participant.hero_class_id = Some(8);
                } else if WARLOCK_SPELLS.contains(&spell_id) {
                    participant.hero_class_id = Some(9);
                } else if DRUID_SPELLS.contains(&spell_id) {
                    participant.hero_class_id = Some(11);
                }
            }
        }
    }

    pub fn is_owner_binding_pet_ability(&self, spell_id: u32) -> bool {
        lazy_static! {
            static ref ABILITIES: BTreeSet<u32> = [25228, 27046, 43771, 6991].iter().cloned().collect();
        }
        ABILITIES.contains(&spell_id)
    }

    pub fn collect_active_map(&mut self, data: &Data, unit: &Unit, now: u64) {
        self.active_map.collect(data, unit, 2, now);
    }

    fn parse_armory(&mut self, armory_content: String) -> Option<()> {
        lazy_static! {
            static ref RE_PLAYER_INFOS: Regex = Regex::new(r##"\["0x([A-Z0-9]+)"\]\s=\s"([^"]+)","##).unwrap();
            static ref RE_LOOT: Regex = Regex::new(r##""(.+)&LOOT&(.+[^\s]) receives loot: \|c([a-zA-Z0-9]+)\|Hitem:(\d+):(.+)\|h\[([a-zA-Z0-9\s']+)\]\|h\|rx(\d+)\.","##).unwrap();
            static ref RE_ZONE: Regex = Regex::new(r##"(.+)&ZONE_INFO&([^&]+)&([^&]+)&?([^"]*)"##).unwrap();
        }

        for cap in RE_PLAYER_INFOS.captures_iter(&armory_content) {
            let args: Vec<&str> = cap[2].split('&').collect();
            if let Ok(unit_guid) = u64::from_str_radix(args[0].trim_start_matches("0x"), 16) {
                let unit_name = args[1].to_string();
                let race = args[2];
                let hero_class = args[3];
                let gender = args[4];
                let guild_name = args[5];
                let guild_rank_name = args[6];
                let guild_rank_index = args[7];
                let pet_guids_str = args[8];
                let gear = args[9];
                let talents = args[10];
                let _arena_team2 = args[11];
                let _arena_team3 = args[12];
                let _arena_team5 = args[13];

                let mut participant = Participant::new(unit_guid, true, unit_name, 0);

                if race != "nil" {
                    participant.race_id = Some(match race {
                        "Human" => 1,
                        "Orc" => 2,
                        "Dwarf" => 3,
                        "Night Elf" => 4,
                        "NightElf" => 4,
                        "Undead" => 5,
                        "Scourge" => 5,
                        "Tauren" => 6,
                        "Gnome" => 7,
                        "Troll" => 8,
                        "Blood Elf" => 10,
                        "Draenei" => 11,
                        _ => return None,
                    });
                }

                if hero_class != "nil" {
                    participant.hero_class_id = Some(match hero_class {
                        "Warrior" => 1,
                        "Paladin" => 2,
                        "Hunter" => 3,
                        "Rogue" => 4,
                        "Priest" => 5,
                        "Death Knight" => 6,
                        "Shaman" => 7,
                        "Mage" => 8,
                        "Warlock" => 9,
                        "Druid" => 11,
                        _ => return None,
                    });
                }

                if gender != "nil" {
                    if gender == "2" {
                        participant.gender_id = Some(false);
                    } else if gender == "3" {
                        participant.gender_id = Some(true);
                    }
                }

                if guild_name != "nil" && guild_rank_name != "nil" && guild_rank_index != "nil" {
                    participant.guild_args = Some((guild_name.to_string(), guild_rank_name.to_string(), u8::from_str_radix(guild_rank_index, 10).ok()?));
                }

                let items: Vec<&str> = gear.split('}').collect();
                if items.iter().any(|item| *item != "nil") {
                    let mut gear = Vec::with_capacity(19);
                    let gear_setups = participant.gear_setups.get_or_insert_with(Vec::new);
                    for item in items {
                        if item == "nil" {
                            gear.push(None);
                            continue;
                        }

                        let item_args = item.split(':').collect::<Vec<&str>>();
                        let item_id = u32::from_str_radix(item_args[0], 10).ok()?;
                        let enchant_id = u32::from_str_radix(item_args[1], 10).ok()?;
                        let mut gems = Vec::with_capacity(4);
                        for arg in item_args.iter().take(6).skip(2) {
                            let gem_id = u32::from_str_radix(arg, 10).ok()?;
                            if gem_id > 0 {
                                gems.push(Some(gem_id));
                            } else {
                                gems.push(None);
                            }
                        }

                        if item_id == 0 {
                            gear.push(None);
                        } else if enchant_id == 0 {
                            gear.push(Some((item_id, None, Some(gems))));
                        } else {
                            gear.push(Some((item_id, Some(enchant_id), Some(gems))));
                        }
                    }
                    gear_setups.push((now() * 1000, gear));
                }

                if talents != "nil" {
                    participant.talents = strip_talent_specialization(&Some(talents.replace("}", "|")));
                }

                self.participants.insert(unit_guid, participant);

                // Create Spell Summon events
                if pet_guids_str != "nil" {
                    for pet_guid_str in pet_guids_str.split('}') {
                        if let Ok(mut pet_unit_guid) = u64::from_str_radix(pet_guid_str.trim_start_matches("0x"), 16) {
                            // TODO: Do we need to add instance events?
                            if pet_unit_guid.is_pet() {
                                let mut new_unit_id = pet_unit_guid;
                                new_unit_id = (new_unit_id & 0x000000FFFF000000).rotate_right(24);
                                new_unit_id |= 0x000000FFFF000000;
                                new_unit_id |= 0xF140000000000000;
                                pet_unit_guid = new_unit_id;
                            }

                            self.bonus_messages.push(Message::new_parsed(
                                10000, 1,
                                MessageType::Summon(
                                    Summon {
                                        owner: Unit { is_player: true, unit_id: unit_guid },
                                        unit: Unit { is_player: false, unit_id: pet_unit_guid },
                                    })));
                        }
                    }
                }
            }
        }

        for cap in RE_LOOT.captures_iter(&armory_content) {
            let unit_name = cap[2].to_string();
            if let Some(unit_guid) = self.participants.iter().find_map(|(guid, participant)| if participant.name == unit_name { Some(*guid) } else { None }) {
                if let Ok(timestamp) = NaiveDateTime::parse_from_str(&cap[1], "%d.%m.%y %H:%M:%S") {
                    if let Ok(item_id) = u32::from_str_radix(&cap[4], 10) {
                        if let Ok(count) = u32::from_str_radix(&cap[7], 10) {
                            self.bonus_messages.push(Message::new_parsed(timestamp.timestamp_millis() as u64, 0, MessageType::Loot(Loot {
                                unit: Unit { is_player: true, unit_id: unit_guid },
                                item_id,
                                count,
                            })));
                        }
                    }
                }
            }
        }

        for cap in RE_ZONE.captures_iter(&armory_content) {
            if let Ok(timestamp) = NaiveDateTime::parse_from_str(&cap[1], "%d.%m.%y %H:%M:%S") {
                if let Ok(instance_id) = u32::from_str_radix(&cap[2], 10) {
                    let map_id = match &cap[2] {
                        "Karazhan" => 532,
                        "Magtheridon's Lair" => 544,
                        "Gruul's Lair" => 565,
                        "Coilfang: Serpentshrine Cavern" => 548,
                        "Serpentshrine Cavern" => 548,
                        "Tempest Keep" => 550,
                        "Black Temple" => 564,
                        "The Battle for Mount Hyjal" => 534,
                        "The Sunwell" => 580,
                        _ => continue
                    };
                    if !cap[3].is_empty() {
                        for participant_unit_guid in cap[3].split('&') {
                            if let Ok(guid) = u64::from_str_radix(participant_unit_guid.trim_start_matches("0x"), 16) {
                                self.bonus_messages.push(Message::new_parsed(
                                    timestamp.timestamp_millis() as u64, 0,
                                    MessageType::InstanceMap(
                                        InstanceMap {
                                            map_id,
                                            instance_id,
                                            map_difficulty: 0,
                                            unit: Unit { is_player: true, unit_id: guid },
                                        })));
                            }
                        }
                    }
                }
            }
        }


        Some(())
    }
}
