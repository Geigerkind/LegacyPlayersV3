use crate::modules::armory::tools::strip_talent_specialization;
use crate::modules::data::Data;
use crate::modules::live_data_processor::dto::{InstanceMap, Loot, Message, MessageType, Summon, Unit};
use crate::modules::live_data_processor::material::{ActiveMapMap, CollectActiveMap, Participant};
use crate::modules::live_data_processor::tools::GUID;
use chrono::NaiveDateTime;
use regex::Regex;
use std::collections::{BTreeSet, HashMap};
use time_util::now;

pub struct WoWWOTLKParser {
    pub server_id: u32,

    pub participants: HashMap<u64, Participant>,
    pub active_map: ActiveMapMap,

    // Hacky
    pub bonus_messages: Vec<Message>,
}

impl WoWWOTLKParser {
    pub fn new(server_id: u32, armory_content: Option<String>) -> Self {
        let mut parser = WoWWOTLKParser {
            server_id,
            participants: Default::default(),
            active_map: Default::default(),
            bonus_messages: Default::default(),
        };
        if let Some(armory_content) = armory_content {
            let _ = parser.parse_armory(armory_content);
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
            static ref WARRIOR_SPELLS: BTreeSet<u32> = [47450, 57755, 23881, 23880, 23885, 12721, 12721, 1680, 44949, 46916, 50783, 47488, 47498, 57823, 46968, 47520, 7384, 20647, 47502]
                .iter()
                .cloned()
                .collect();
            static ref PALADIN_SPELLS: BTreeSet<u32> =
                [53739, 20167, 20424, 20375, 53736, 20165, 20166, 20271, 53733, 20186, 68066, 54158, 20185, 54153, 20267, 20467, 53408, 48819, 53385, 54172, 35395, 53742, 61411, 61840, 53595, 48801, 48952, 48827, 61411, 48782, 54968]
                    .iter()
                    .cloned()
                    .collect();
            static ref ROGUE_SPELLS: BTreeSet<u32> = [57965, 57993, 57970, 48665, 48664, 48666, 51723, 52874, 48676, 48691].iter().cloned().collect();
            static ref PRIEST_SPELLS: BTreeSet<u32> = [52985, 56160, 48071, 33110, 48156, 58381, 48160, 48300, 48127, 48125, 63675, 75999, 53022, 53023, 48072, 48068, 64844].iter().cloned().collect();
            static ref HUNTER_SPELLS: BTreeSet<u32> = [53352, 60053, 75, 49052, 61006, 49001, 63672, 49050, 58433, 58434, 49065, 49048].iter().cloned().collect();
            static ref MAGE_SPELLS: BTreeSet<u32> = [42897, 36032, 42845, 42939, 42940, 42937, 42938, 59638, 55802, 55807, 59637, 57984, 122, 57761, 54096, 42833, 42926, 42931, 42891, 59638]
                .iter()
                .cloned()
                .collect();
            static ref WARLOCK_SPELLS: BTreeSet<u32> = [47813, 47809, 47855, 47843, 47864, 59164, 47210, 47836, 47834, 47825, 47811, 47838, 47867, 50590].iter().cloned().collect();
            static ref SHAMAN_SPELLS: BTreeSet<u32> = [10444, 61654, 49238, 49271, 54531, 58735, 60103, 49233, 49279, 49281, 32175, 32176, 17364, 25504, 49231, 55459, 49276].iter().cloned().collect();
            static ref DRUID_SPELLS: BTreeSet<u32> = [48441, 53251, 48443, 50464, 48461, 48465, 53201, 53195, 53190, 48463, 48468, 48572, 49800, 48574, 48577, 48566, 62078, 48480, 48562, 48564]
                .iter()
                .cloned()
                .collect();
            static ref DEATH_KNIGHT_SPELLS: BTreeSet<u32> = [55268, 66962, 51425, 66974, 51411, 55095, 55078, 51460, 52212, 49938, 49909, 49921, 66992, 47632, 50526, 55095, 61696, 49930, 66979]
                .iter()
                .cloned()
                .collect();
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
                } else if DEATH_KNIGHT_SPELLS.contains(&spell_id) {
                    participant.hero_class_id = Some(6);
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
            static ref ABILITIES: BTreeSet<u32> = [25228, 27046, 43771, 6991, 48990, 43771].iter().cloned().collect();
        }
        ABILITIES.contains(&spell_id)
    }

    pub fn collect_active_map(&mut self, data: &Data, unit: &Unit, now: u64) {
        self.active_map.collect(data, unit, 3, now);
    }

    fn parse_armory(&mut self, armory_content: String) -> Option<()> {
        lazy_static! {
            static ref RE_PLAYER_INFOS: Regex = Regex::new(r##"\["0x([A-Z0-9]+)"\]\s=\s"([^"]+)","##).unwrap();
            static ref RE_LOOT: Regex = Regex::new(r##""(.+)&LOOT&(.+[^\s]) receives loot: \|c([a-zA-Z0-9]+)\|Hitem:(\d+):(.+)\|h\[([a-zA-Z0-9\s']+)\]\|h\|rx(\d+)\.","##).unwrap();
            static ref RE_ZONE: Regex = Regex::new(r##"(.+)&ZONE_INFO&([^&]+)&([^&]+)&([^&]+)&([^&]+)&([^&]+)&([^&]+)&([^&]+)&?([^"]*)"##).unwrap();
        }

        for cap in RE_PLAYER_INFOS.captures_iter(&armory_content) {
            let args: Vec<&str> = cap[2].split('&').collect();
            if let Ok(mut unit_guid) = u64::from_str_radix(args[0].trim_start_matches("0x"), 16) {
                // Crystalsong UID fix
                if unit_guid.get_high() == 0x0110 {
                    unit_guid &= 0x0000000000FFFFFF;
                }

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
                            if pet_unit_guid.is_pet() {
                                let mut new_unit_id = pet_unit_guid;
                                new_unit_id = (new_unit_id & 0x000000FFFF000000).rotate_right(24);
                                new_unit_id |= 0x000000FFFF000000;
                                new_unit_id |= 0xF140000000000000;
                                pet_unit_guid = new_unit_id;
                            }

                            self.bonus_messages.push(Message::new_parsed(
                                10000,
                                1,
                                MessageType::Summon(Summon {
                                    owner: Unit { is_player: true, unit_id: unit_guid },
                                    unit: Unit { is_player: false, unit_id: pet_unit_guid },
                                }),
                            ));
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
                            self.bonus_messages.push(Message::new_parsed(
                                timestamp.timestamp_millis() as u64,
                                0,
                                MessageType::Loot(Loot {
                                    unit: Unit { is_player: true, unit_id: unit_guid },
                                    item_id,
                                    count,
                                }),
                            ));
                        }
                    }
                }
            }
        }

        for cap in RE_ZONE.captures_iter(&armory_content) {
            if let Ok(timestamp) = NaiveDateTime::parse_from_str(&cap[1], "%d.%m.%y %H:%M:%S") {
                if &cap[9] != "nil" {
                    if let Ok(map_id) = u32::from_str_radix(&cap[8], 10) {
                        if let Ok(instance_id) = u32::from_str_radix(&cap[9], 10) {
                            let difficulty_id = match &cap[5] {
                                "10 Player" => 3,
                                "25 Player" => 4,
                                "10 Player (Heroic)" => 5,
                                "25 Player (Heroic)" => 6,
                                _ => continue,
                            };
                            if !cap[10].is_empty() {
                                for participant_unit_guid in cap[10].split('&') {
                                    if let Ok(mut guid) = u64::from_str_radix(participant_unit_guid.trim_start_matches("0x"), 16) {
                                        // Crystalsong UID fix
                                        if guid.get_high() == 0x0110 {
                                            guid &= 0x0000000000FFFFFF;
                                        }
                                        self.bonus_messages.push(Message::new_parsed(
                                            timestamp.timestamp_millis() as u64,
                                            0,
                                            MessageType::InstanceMap(InstanceMap {
                                                map_id,
                                                instance_id,
                                                map_difficulty: difficulty_id,
                                                unit: Unit { is_player: true, unit_id: guid },
                                            }),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Some(())
    }
}
