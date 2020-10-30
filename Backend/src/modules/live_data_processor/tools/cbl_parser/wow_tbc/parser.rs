use crate::modules::armory::dto::{CharacterDto, CharacterHistoryDto, CharacterInfoDto, GuildDto, CharacterGuildDto, CharacterGearDto, CharacterItemDto};
use crate::modules::data::Data;
use crate::modules::live_data_processor::domain_value::HitType;
use crate::modules::live_data_processor::dto::{AuraApplication, DamageDone, Death, HealDone, Interrupt, Message, MessageType, SpellCast, Summon, UnAura, Unit};
use crate::modules::live_data_processor::material::{ActiveMapVec, Participant, WoWTBCParser};
use crate::modules::live_data_processor::tools::cbl_parser::combat_log_parser::CombatLogParser;
use crate::modules::live_data_processor::tools::cbl_parser::wow_tbc::parse_damage::parse_damage;
use crate::modules::live_data_processor::tools::cbl_parser::wow_tbc::parse_heal::parse_heal;
use crate::modules::live_data_processor::tools::cbl_parser::wow_tbc::parse_miss::parse_miss;
use crate::modules::live_data_processor::tools::cbl_parser::wow_tbc::parse_spell_args::parse_spell_args;
use crate::modules::live_data_processor::tools::cbl_parser::wow_tbc::parse_unit::parse_unit;
use crate::modules::armory::domain_value::GuildRank;
use crate::util::hash_str::hash_str;

impl CombatLogParser for WoWTBCParser {
    fn parse_cbl_line(&mut self, data: &Data, event_ts: u64, content: &str) -> Option<Vec<MessageType>> {
        let message_args = content.trim_end_matches('\r').split(',').collect::<Vec<&str>>();
        Some(match message_args[0] {
            "SWING_DAMAGE" => {
                let attacker = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let victim = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let (hit_mask, blocked, damage_component) = parse_damage(&message_args[7..])?;
                self.collect_participant(&attacker, message_args[2], event_ts);
                self.collect_participant(&victim, message_args[5], event_ts);
                self.collect_active_map(data, &attacker, event_ts);
                self.collect_active_map(data, &victim, event_ts);
                self.participants.get_mut(&victim.unit_id).unwrap().attribute_damage(damage_component.damage);
                vec![MessageType::MeleeDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: None,
                    hit_mask,
                    blocked,
                    damage_over_time: false,
                    damage_components: vec![damage_component],
                })]
            }
            "SWING_MISSED" => {
                let attacker = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let victim = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let hit_mask = parse_miss(&message_args[7..])?;
                self.collect_participant(&attacker, message_args[2], event_ts);
                self.collect_participant(&victim, message_args[5], event_ts);
                self.collect_active_map(data, &attacker, event_ts);
                self.collect_active_map(data, &victim, event_ts);
                vec![MessageType::MeleeDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: None,
                    hit_mask,
                    blocked: 0,
                    damage_over_time: false,
                    damage_components: Vec::new(),
                })]
            }
            "SPELL_DAMAGE" | "SPELL_PERIODIC_DAMAGE" | "RANGE_DAMAGE" | "DAMAGE_SHIELD" | "DAMAGE_SPLIT" => {
                let attacker = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let victim = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let spell_id = parse_spell_args(&message_args[7..10])?;
                let (hit_mask, blocked, damage_component) = parse_damage(&message_args[10..])?;
                self.collect_participant(&attacker, message_args[2], event_ts);
                self.collect_participant(&victim, message_args[5], event_ts);
                self.collect_participant_class(&attacker, spell_id);
                self.collect_active_map(data, &attacker, event_ts);
                self.collect_active_map(data, &victim, event_ts);
                self.participants.get_mut(&victim.unit_id).unwrap().attribute_damage(damage_component.damage);
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
            }
            "SPELL_MISSED" | "SPELL_PERIODIC_MISSED" | "RANGE_MISSED" | "DAMAGE_SHIELD_MISSED" => {
                let attacker = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let victim = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let spell_id = parse_spell_args(&message_args[7..10])?;
                let hit_mask = parse_miss(&message_args[10..])?;
                self.collect_participant(&attacker, message_args[2], event_ts);
                self.collect_participant(&victim, message_args[5], event_ts);
                self.collect_participant_class(&attacker, spell_id);
                self.collect_active_map(data, &attacker, event_ts);
                self.collect_active_map(data, &victim, event_ts);
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
                        blocked: 0,
                        damage_over_time: false, // message_args[0] == "SPELL_PERIODIC_MISSED",
                        damage_components: Vec::new(),
                    }),
                ]
            }
            "SPELL_HEAL" | "SPELL_PERIODIC_HEAL" => {
                let caster = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let target = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let spell_id = parse_spell_args(&message_args[7..10])?;
                let (amount, is_crit) = parse_heal(&message_args[10..])?;
                self.collect_participant(&caster, message_args[2], event_ts);
                self.collect_participant(&target, message_args[5], event_ts);
                self.collect_participant_class(&caster, spell_id);
                let effective_heal = self.participants.get_mut(&target.unit_id).unwrap().attribute_heal(amount);
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
                        effective_heal,
                        absorb: 0,
                        hit_mask: if is_crit { HitType::Crit as u32 } else { HitType::Hit as u32 },
                    }),
                ]
            }
            // "SPELL_AURA_APPLIED_DOSE" | "SPELL_AURA_REMOVED_DOSE"
            "SPELL_AURA_APPLIED" | "SPELL_AURA_REMOVED" => {
                let caster = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let target = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let spell_id = parse_spell_args(&message_args[7..10])?;
                let is_removed = message_args[0].contains("REMOVED");
                self.collect_participant(&caster, message_args[2], event_ts);
                self.collect_participant(&target, message_args[5], event_ts);
                self.collect_active_map(data, &caster, event_ts);
                self.collect_active_map(data, &target, event_ts);
                let mut result = vec![MessageType::AuraApplication(AuraApplication {
                    caster: caster.clone(),
                    target: target.clone(),
                    spell_id,
                    stack_amount: if is_removed { 0 } else { 1 }, // TODO: Amount estimation
                    delta: if is_removed { -1 } else { 1 },
                })];

                if self.is_owner_binding_pet_ability(spell_id) {
                    result.push(MessageType::Summon(Summon { owner: target, unit: caster }));
                }

                result
            }
            "SPELL_CAST_SUCCESS" => {
                let caster = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let target = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let spell_id = parse_spell_args(&message_args[7..10])?;
                self.collect_participant(&caster, message_args[2], event_ts);
                self.collect_participant(&target, message_args[5], event_ts);
                self.collect_active_map(data, &caster, event_ts);
                self.collect_active_map(data, &target, event_ts);

                let mut result = vec![MessageType::SpellCast(SpellCast {
                    caster: caster.clone(),
                    target: Some(target.clone()),
                    spell_id,
                    hit_mask: HitType::Hit as u32,
                })];

                if self.is_owner_binding_pet_ability(spell_id) {
                    result.push(MessageType::Summon(Summon { owner: caster, unit: target }));
                }

                result
            }
            "SPELL_SUMMON" => {
                let owner = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let unit = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                vec![MessageType::Summon(Summon { owner, unit })]
            }
            "UNIT_DIED" | "UNIT_DESTROYED" => {
                let victim = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                vec![MessageType::Death(Death { cause: None, victim })]
            }
            "SPELL_DISPEL" => {
                let un_aura_caster = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let target = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let un_aura_spell_id = parse_spell_args(&message_args[7..10])?;
                let target_spell_id = parse_spell_args(&message_args[10..13])?;
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
            }
            "SPELL_INTERRUPT" => {
                let un_aura_caster = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let target = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let un_aura_spell_id = parse_spell_args(&message_args[7..10])?;
                let interrupted_spell_id = parse_spell_args(&message_args[10..13])?;
                vec![
                    MessageType::SpellCast(SpellCast {
                        caster: un_aura_caster,
                        target: Some(target.clone()),
                        spell_id: un_aura_spell_id,
                        hit_mask: HitType::Hit as u32,
                    }),
                    MessageType::Interrupt(Interrupt { target, interrupted_spell_id }),
                ]
            }
            "SPELL_STOLEN" => {
                let un_aura_caster = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let target = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let un_aura_spell_id = parse_spell_args(&message_args[7..10])?;
                let target_spell_id = parse_spell_args(&message_args[10..13])?;
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
            }
            // TODO: Use more events
            // https://wow.gamepedia.com/COMBAT_LOG_EVENT?oldid=1585715
            _ => return None,
        })
    }

    fn do_message_post_processing(&mut self, _data: &Data, _messages: &mut Vec<Message>) {
        // Do nothing
    }

    fn get_involved_server(&self) -> Option<Vec<(u32, String, String)>> {
        None
    }

    fn get_involved_character_builds(&self) -> Vec<(Option<u32>, u64, CharacterDto)> {
        self.participants
            .iter()
            .filter(|(_, participant)| participant.is_player)
            .fold(Vec::new(), |mut acc, (_, participant)| {
                let gear_setups = &participant.gear_setups;
                if gear_setups.is_some() && !gear_setups.as_ref().unwrap().is_empty() {
                    for (ts, gear) in gear_setups.as_ref().unwrap().iter() {
                        acc.push((
                            None,
                            *ts,
                            CharacterDto {
                                server_uid: participant.id,
                                character_history: Some(CharacterHistoryDto {
                                    character_info: CharacterInfoDto {
                                        gear: CharacterGearDto {
                                            head: create_character_item_dto(&gear[0]),
                                            neck: create_character_item_dto(&gear[1]),
                                            shoulder: create_character_item_dto(&gear[2]),
                                            back: create_character_item_dto(&gear[14]),
                                            chest: create_character_item_dto(&gear[4]),
                                            shirt: create_character_item_dto(&gear[3]),
                                            tabard: create_character_item_dto(&gear[18]),
                                            wrist: create_character_item_dto(&gear[8]),
                                            main_hand: create_character_item_dto(&gear[15]),
                                            off_hand: create_character_item_dto(&gear[16]),
                                            ternary_hand: create_character_item_dto(&gear[17]),
                                            glove: create_character_item_dto(&gear[9]),
                                            belt: create_character_item_dto(&gear[5]),
                                            leg: create_character_item_dto(&gear[6]),
                                            boot: create_character_item_dto(&gear[7]),
                                            ring1: create_character_item_dto(&gear[10]),
                                            ring2: create_character_item_dto(&gear[11]),
                                            trinket1: create_character_item_dto(&gear[12]),
                                            trinket2: create_character_item_dto(&gear[13]),
                                        },
                                        hero_class_id: participant.hero_class_id.unwrap_or(12),
                                        level: 70,
                                        gender: participant.gender_id.unwrap_or(false),
                                        profession1: None,
                                        profession2: None,
                                        talent_specialization: participant.talents.clone(),
                                        race_id: participant.race_id.unwrap_or(1),
                                    },
                                    character_name: participant.name.clone(),
                                    character_guild: participant.guild_args.as_ref().map(|(guild_name, rank_name, rank_index)| CharacterGuildDto {
                                        guild: GuildDto {
                                            server_uid: hash_str(guild_name) & 0x0000FFFFFFFFFFFF,
                                            name: guild_name.clone(),
                                        },
                                        rank: GuildRank { index: *rank_index, name: rank_name.clone() },
                                    }),
                                    character_title: None,
                                    profession_skill_points1: None,
                                    profession_skill_points2: None,
                                    facial: None,
                                    arena_teams: vec![],
                                }),
                            },
                        ));
                    }
                } else {
                    acc.push((
                        None,
                        time_util::now() * 1000,
                        CharacterDto {
                            server_uid: participant.id,
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
                                    hero_class_id: participant.hero_class_id.unwrap_or(1),
                                    level: 70,
                                    gender: participant.gender_id.unwrap_or(false),
                                    profession1: None,
                                    profession2: None,
                                    talent_specialization: participant.talents.clone(),
                                    race_id: participant.race_id.unwrap_or(1),
                                },
                                character_name: participant.name.clone(),
                                character_guild: participant.guild_args.as_ref().map(|(guild_name, rank_name, rank_index)| CharacterGuildDto {
                                    guild: GuildDto {
                                        server_uid: hash_str(guild_name) & 0x0000FFFFFFFFFFFF,
                                        name: guild_name.clone(),
                                    },
                                    rank: GuildRank { index: *rank_index, name: rank_name.clone() },
                                }),
                                character_title: None,
                                profession_skill_points1: None,
                                profession_skill_points2: None,
                                facial: None,
                                arena_teams: vec![],
                            }),
                        },
                    ));
                }
                acc
            })
    }

    fn get_participants(&self) -> Vec<Participant> {
        self.participants.iter().map(|(_, participant)| participant).cloned().collect()
    }

    fn get_active_maps(&self) -> ActiveMapVec {
        self.active_map.iter().map(|(_, active_map)| active_map.clone()).collect()
    }

    fn get_npc_appearance_offset(&self, entry: u32) -> Option<i64> {
        Some(match entry {
            20060 | 20062 | 20063 | 20064 | 19622 | 21268 | 21269 | 21270 | 21271 | 21272 | 21273 | 21274 => -180000,
            _ => return None
        })
    }

    fn get_npc_timeout(&self, entry: u32) -> Option<u64> {
        Some(match entry {
            20060 | 20062 | 20063 | 20064 | 19622 | 21268 | 21269 | 21270 | 21271 | 21272 | 21273 | 21274 => 180000,
            _ => return None
        })
    }

    fn get_death_implied_npc_combat_state_and_offset(&self, entry: u32) -> Option<Vec<(u32, i64)>> {
        Some(match entry {
            20060 => vec![(19622, -1000), (20062, -1000), (20063, -1000), (20064, -1000), (21268, -1000), (21269, -1000), (21270, -1000), (21271, -1000), (21272, -1000), (21273, -1000), (21274, -1000)],
            20062 => vec![(19622, -1000), (20063, -1000), (20064, -1000), (21268, -1000), (21269, -1000), (21270, -1000), (21271, -1000), (21272, -1000), (21273, -1000), (21274, -1000)],
            20063 => vec![(19622, -1000), (20064, -1000), (21268, -1000), (21269, -1000), (21270, -1000), (21271, -1000), (21272, -1000), (21273, -1000), (21274, -1000)],
            20064 | 18545 | 21268 | 21269 | 21270 | 21271 | 21272 | 21273 | 21274 => vec![(19622, -1000)],
            _ => return None
        })
    }

    fn get_in_combat_implied_npc_combat(&self, entry: u32) -> Option<Vec<u32>> {
        Some(match entry {
            20060 => vec![19622, 20062, 20063, 20064, 21268, 21269, 21270, 21271, 21272, 21273, 21274],
            20062 => vec![19622, 20063, 20064, 21268, 21268, 21269, 21270, 21271, 21272, 21273, 21274],
            20063 => vec![19622, 20064, 21268, 21268, 21269, 21270, 21271, 21272, 21273, 21274],
            18545 | 21268 | 21269 | 21270 | 21271 | 21272 | 21273 | 21274 => vec![19622],
            _ => return None
        })
    }

    fn get_expansion_id(&self) -> u8 {
        2
    }

    fn get_server_id(&self) -> Option<u32> {
        Some(self.server_id)
    }

    fn get_bonus_messages(&self) -> Option<Vec<Message>> {
        Some(self.bonus_messages.clone())
    }
}

fn create_character_item_dto(item: &Option<(u32, Option<u32>, Option<Vec<Option<u32>>>)>) -> Option<CharacterItemDto> {
    item.as_ref().map(|(item_id, enchant_id, gems)| CharacterItemDto {
        item_id: *item_id,
        random_property_id: None,
        enchant_id: enchant_id.clone(),
        gem_ids: gems.clone().unwrap_or_else(Vec::new),
    })
}