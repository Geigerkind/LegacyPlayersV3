use crate::modules::armory::dto::{CharacterDto, CharacterGearDto, CharacterHistoryDto, CharacterInfoDto};
use crate::modules::armory::tools::{GetCharacter, SetCharacter};
use crate::modules::armory::Armory;
use crate::modules::data::Data;
use crate::modules::live_data_processor::domain_value::{HitType, School};
use crate::modules::live_data_processor::dto::{AuraApplication, CombatState, DamageComponent, DamageDone, Death, HealDone, InstanceMap, Interrupt, LiveDataProcessorFailure, Message, MessageType, SpellCast, Summon, UnAura, Unit};
use crate::modules::live_data_processor::material::WoWCBTLParser;
use crate::modules::live_data_processor::tools::GUID;
use crate::util::database::{Execute, Select};
use chrono::{Datelike, NaiveDateTime};
use std::collections::{BTreeSet, HashMap};
use std::time::SystemTime;

pub trait LogParser {
    fn parse(&mut self, db_main: &mut (impl Select + Execute), data: &Data, armory: &Armory, log_path: &str) -> Vec<Message>;
}

impl LogParser for WoWCBTLParser {
    fn parse(&mut self, db_main: &mut (impl Select + Execute), data: &Data, armory: &Armory, log_path: &str) -> Vec<Message> {
        let mut messages = Vec::with_capacity(1000000);
        // TODO: Handle 31/12 => 01/01 raids
        let current_year = NaiveDateTime::from_timestamp(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64, 0).year();
        if let Ok(file_content) = std::fs::read_to_string(log_path) {
            for line in file_content.split('\n').into_iter() {
                let meta = line.split("  ").collect::<Vec<&str>>();
                if meta.len() != 2 {
                    continue;
                }
                if let Ok(timestamp) = NaiveDateTime::parse_from_str(&format!("{}/{}", current_year, meta[0]), "%Y/%m/%d %H:%M:%S%.3f") {
                    let message_args = meta[1].trim_end_matches('\r').split(',').collect::<Vec<&str>>();
                    let event_timestamp = timestamp.timestamp_millis() as u64;
                    if let Ok(message_types) = parse_log_message(self, data, event_timestamp, message_args) {
                        let mut message_count = (messages.len() + message_types.len() - 1) as u64;
                        for message_type in message_types {
                            messages.push(Message {
                                api_version: 0,
                                message_length: 0,
                                timestamp: event_timestamp,
                                message_count,
                                message_type,
                            });
                            message_count -= 1;
                        }
                    }
                }
            }
        }

        // Post processing step
        // Insert new participants
        for (unit_id, (unit_name, hero_class_id)) in self.found_player.iter() {
            if armory.get_character_by_uid(self.server_id, *unit_id).is_none() {
                let _ = armory.set_character(
                    db_main,
                    self.server_id,
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

        let mut current_map = None;
        let mut instance_ids = HashMap::new();
        let mut participants = BTreeSet::new();
        let mut additional_messages = Vec::with_capacity(1000);
        let mut last_combat_update = HashMap::new();
        for current_message in messages.iter() {
            let current_timestamp = current_message.timestamp;
            let current_message_count = current_message.message_count;

            // Insert Instance Map Messages
            if let Some((map_id, difficulty)) = get_current_map(self, current_timestamp) {
                let new_participants: Vec<(u64, bool)> = if current_map.contains(&(map_id, difficulty)) {
                    self.participation
                        .iter()
                        .filter_map(|(unit_id, (_, is_player, intervals))| {
                            if participants.contains(&(*unit_id, *is_player)) {
                                return None;
                            }
                            if intervals.iter().any(|(start, end)| *start <= current_timestamp && *end >= current_timestamp) {
                                return Some((*unit_id, *is_player));
                            }
                            None
                        })
                        .collect()
                } else {
                    current_map = Some((map_id, difficulty));
                    self.participation
                        .iter()
                        .filter_map(|(unit_id, (_, is_player, intervals))| {
                            if intervals.iter().any(|(start, end)| *start <= current_timestamp && *end >= current_timestamp) {
                                return Some((*unit_id, *is_player));
                            }
                            None
                        })
                        .collect()
                };

                let instance_id = *instance_ids.entry((map_id, difficulty)).or_insert_with(rand::random::<u32>);
                for (unit_id, is_player) in new_participants {
                    // Thaddius is a special snowflake
                    let mut ts_offset: i64 = -1;
                    if !is_player && unit_id.get_entry().contains(&15928) {
                        ts_offset = -30000;
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
                    participants.insert((unit_id, is_player));
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
        messages.sort_by(|left, right| left.timestamp.cmp(&right.timestamp));
        messages
    }
}

fn add_combat_event(additional_messages: &mut Vec<Message>, last_combat_update: &mut HashMap<u64, u64>, current_timestamp: u64, current_message_count: u64, unit: &Unit) {
    if let Some(last_update) = last_combat_update.get_mut(&unit.unit_id) {
        if current_timestamp - *last_update >= 60000 {
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
                timestamp: current_timestamp - 1,
                message_count: current_message_count,
                message_type: MessageType::CombatState(CombatState { unit: unit.clone(), in_combat: true }),
            });
        }
        *last_update = current_timestamp;
    } else {
        additional_messages.push(Message {
            api_version: 0,
            message_length: 0,
            timestamp: current_timestamp - 1,
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
        let last_index = me.active_difficulty.len() - 1;
        for (index, (pot_diff_id, start, end)) in me.active_difficulty.iter().enumerate() {
            if *start <= current_timestamp && (index == last_index || *end >= current_timestamp) {
                difficulty_id = Some(*pot_diff_id);
                break;
            }
        }

        if difficulty_id.is_none() {
            match map_id {
                533 | 603 | 615 | 616 | 624 => {
                    if me
                        .participation
                        .iter()
                        .filter(|(_unit_id, (_, is_player, intervals))| intervals.iter().any(|(start, end)| *is_player && *start <= current_timestamp && *end >= current_timestamp))
                        .count()
                        > 10
                    {
                        difficulty_id = Some(4);
                    } else {
                        difficulty_id = Some(3);
                    }
                },
                _ => {},
            };
        }

        difficulty_id.map(|difficulty_id| (map_id, Some(difficulty_id)))
    })
}

fn parse_log_message(me: &mut WoWCBTLParser, data: &Data, event_timestamp: u64, message_args: Vec<&str>) -> Result<Vec<MessageType>, LiveDataProcessorFailure> {
    Ok(match message_args[0] {
        "SWING_DAMAGE" => {
            let attacker = parse_unit(me, data, event_timestamp, &message_args[1..4])?;
            let victim = parse_unit(me, data, event_timestamp, &message_args[4..7])?;
            let (hit_mask, blocked, damage_component) = parse_damage(&message_args[7..])?;
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
            let attacker = parse_unit(me, data, event_timestamp, &message_args[1..4])?;
            let victim = parse_unit(me, data, event_timestamp, &message_args[4..7])?;
            let (hit_mask, blocked, damage_component) = parse_miss(&message_args[7..])?;
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
            let attacker = parse_unit(me, data, event_timestamp, &message_args[1..4])?;
            let victim = parse_unit(me, data, event_timestamp, &message_args[4..7])?;
            let (spell_id, _school_mask) = parse_spell_args(me, event_timestamp, &message_args[7..10])?;
            let (hit_mask, blocked, damage_component) = parse_damage(&message_args[10..])?;
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
            let attacker = parse_unit(me, data, event_timestamp, &message_args[1..4])?;
            let victim = parse_unit(me, data, event_timestamp, &message_args[4..7])?;
            let (spell_id, _school_mask) = parse_spell_args(me, event_timestamp, &message_args[7..10])?;
            let (hit_mask, _blocked, _damage_component) = parse_miss(&message_args[10..])?;
            vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker,
                    target: Some(victim),
                    spell_id,
                    hit_mask,
                }), /*
                    , MessageType::SpellDamage(DamageDone {
                        attacker,
                        victim,
                        spell_id: Some(spell_id),
                        hit_mask,
                        blocked,
                        damage_over_time: false, // message_args[0] == "SPELL_PERIODIC_MISSED",
                        damage_components: damage_component.map(|comp| vec![comp]).unwrap_or_else(|| Vec::new()),
                    })
                     */
            ]
        },
        "SPELL_HEAL" | "SPELL_PERIODIC_HEAL" => {
            let caster = parse_unit(me, data, event_timestamp, &message_args[1..4])?;
            let target = parse_unit(me, data, event_timestamp, &message_args[4..7])?;
            let (spell_id, _school_mask) = parse_spell_args(me, event_timestamp, &message_args[7..10])?;
            let (amount, overhealing, absorbtion, is_crit) = parse_heal(&message_args[10..])?;
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
        // "SPELL_AURA_APPLIED_DOSE" | "SPELL_AURA_REMOVED_DOSE"
        "SPELL_AURA_APPLIED" | "SPELL_AURA_REMOVED" => {
            let caster = parse_unit(me, data, event_timestamp, &message_args[1..4])?;
            let target = parse_unit(me, data, event_timestamp, &message_args[4..7])?;
            let (spell_id, _school_mask) = parse_spell_args(me, event_timestamp, &message_args[7..10])?;
            /*
            let amount = if message_args.len() == 12 {
                i8::from_str_radix(message_args[11], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?
            } else { 0 };
             */
            let is_removed = message_args[0].contains("REMOVED");
            vec![MessageType::AuraApplication(AuraApplication {
                caster,
                target,
                spell_id,
                stack_amount: if is_removed { 0 } else { 1 }, // TODO
                delta: if is_removed { -1 } else { 1 },
            })]
        },
        "SPELL_CAST_SUCCESS" => {
            let caster = parse_unit(me, data, event_timestamp, &message_args[1..4])?;
            let target = parse_unit(me, data, event_timestamp, &message_args[4..7])?;
            let (spell_id, _school_mask) = parse_spell_args(me, event_timestamp, &message_args[7..10])?;
            if caster.is_player {
                me.collect_player(caster.unit_id, message_args[2], spell_id);
            }
            vec![MessageType::SpellCast(SpellCast {
                caster,
                target: Some(target),
                spell_id,
                hit_mask: HitType::Hit as u32,
            })]
        },
        "SPELL_SUMMON" => {
            let owner = parse_unit(me, data, event_timestamp, &message_args[1..4])?;
            let unit = parse_unit(me, data, event_timestamp, &message_args[4..7])?;
            // TODO: Is that of interest?
            // let (spell_id, _school_mask) = parse_spell_args(m, event_timestamp, &message_args[7..10])?;
            vec![MessageType::Summon(Summon { owner, unit })]
        },
        "UNIT_DIED" | "UNIT_DESTROYED" => {
            let victim = parse_unit(me, data, event_timestamp, &message_args[4..7])?;
            vec![MessageType::Death(Death { cause: None, victim })]
        },
        "SPELL_DISPEL" => {
            let un_aura_caster = parse_unit(me, data, event_timestamp, &message_args[1..4])?;
            let target = parse_unit(me, data, event_timestamp, &message_args[4..7])?;
            let (un_aura_spell_id, _school_mask) = parse_spell_args(me, event_timestamp, &message_args[7..10])?;
            let (target_spell_id, _target_school_mask) = parse_spell_args(me, event_timestamp, &message_args[10..13])?;
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
            let un_aura_caster = parse_unit(me, data, event_timestamp, &message_args[1..4])?;
            let target = parse_unit(me, data, event_timestamp, &message_args[4..7])?;
            let (un_aura_spell_id, _school_mask) = parse_spell_args(me, event_timestamp, &message_args[7..10])?;
            let (interrupted_spell_id, _target_school_mask) = parse_spell_args(me, event_timestamp, &message_args[10..13])?;
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
        "SPELL_STOLEN" => {
            let un_aura_caster = parse_unit(me, data, event_timestamp, &message_args[1..4])?;
            let target = parse_unit(me, data, event_timestamp, &message_args[4..7])?;
            let (un_aura_spell_id, _school_mask) = parse_spell_args(me, event_timestamp, &message_args[7..10])?;
            let (target_spell_id, _target_school_mask) = parse_spell_args(me, event_timestamp, &message_args[10..13])?;
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

fn parse_heal(heal_args: &[&str]) -> Result<(u32, u32, u32, bool), LiveDataProcessorFailure> {
    let amount = u32::from_str_radix(heal_args[0], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let overhealing = u32::from_str_radix(heal_args[1], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    // TODO: Use as absorbed hint
    let absorbed = u32::from_str_radix(heal_args[2], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let critical = heal_args[2].starts_with('1');
    Ok((amount, overhealing, absorbed, critical))
}

fn parse_spell_args(me: &mut WoWCBTLParser, event_timestamp: u64, spell_args: &[&str]) -> Result<(u32, u8), LiveDataProcessorFailure> {
    let spell_id = u32::from_str_radix(spell_args[0], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let school_mask = u8::from_str_radix(spell_args[2].trim_start_matches("0x"), 16).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    if let Some(difficulty_id) = me.get_difficulty_by_spell_id(spell_id) {
        if let Some((last_difficulty_id, _, last_end_ts)) = me.active_difficulty.last_mut() {
            if *last_difficulty_id == difficulty_id {
                *last_end_ts = event_timestamp;
            }
        } else {
            // We assume the first entry we see is the truth
            me.active_difficulty.push((difficulty_id, 0, event_timestamp));
        }
    }
    Ok((spell_id, school_mask))
}

fn parse_miss(miss_args: &[&str]) -> Result<(u32, u32, Option<DamageComponent>), LiveDataProcessorFailure> {
    let amount_missed = if miss_args.len() == 2 {
        u32::from_str_radix(miss_args[1], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?
    } else {
        0
    };
    Ok(match miss_args[0] {
        "ABSORB" => (
            HitType::FullAbsorb as u32,
            0,
            Some(DamageComponent {
                school_mask: School::Physical as u8,
                damage: 0,
                resisted_or_glanced: 0,
                absorbed: amount_missed,
            }),
        ),
        "RESIST" => (
            HitType::FullResist as u32,
            0,
            Some(DamageComponent {
                school_mask: School::Physical as u8,
                damage: 0,
                resisted_or_glanced: amount_missed,
                absorbed: 0,
            }),
        ),
        "BLOCK" => (HitType::FullBlock as u32, amount_missed, None),
        "DEFLECT" => (HitType::Deflect as u32, 0, None),
        "DODGE" => (HitType::Dodge as u32, 0, None),
        "EVADE" => (HitType::Evade as u32, 0, None),
        "IMMUNE" => (HitType::Immune as u32, 0, None),
        "MISS" => (HitType::Miss as u32, 0, None),
        "PARRY" => (HitType::Parry as u32, 0, None),
        "REFLECT" => (HitType::Reflect as u32, 0, None),
        _ => return Err(LiveDataProcessorFailure::InvalidInput),
    })
}

fn parse_damage(damage_args: &[&str]) -> Result<(u32, u32, DamageComponent), LiveDataProcessorFailure> {
    let amount = u32::from_str_radix(damage_args[0], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let school_mask = u8::from_str_radix(damage_args[2], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let resisted = u32::from_str_radix(damage_args[3], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let blocked = u32::from_str_radix(damage_args[4], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let absorbed = u32::from_str_radix(damage_args[5], 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let critical = damage_args[6].starts_with('1');
    let glancing = damage_args[7].starts_with('1');
    let crushing = damage_args[8].starts_with('1');
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
    // let is_player = (u32::from_str_radix(unit_args[2].trim_start_matches("0x"), 16).map_err(|_| LiveDataProcessorFailure::InvalidInput)? & 0x400) != 0;
    let unit_id = u64::from_str_radix(unit_args[0].trim_start_matches("0x"), 16).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
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
