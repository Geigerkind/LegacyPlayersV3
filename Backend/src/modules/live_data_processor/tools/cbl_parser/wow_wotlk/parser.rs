use crate::modules::armory::dto::{CharacterDto, CharacterHistoryDto, CharacterInfoDto};
use crate::modules::data::Data;
use crate::modules::live_data_processor::domain_value::HitType;
use crate::modules::live_data_processor::dto::{AuraApplication, DamageDone, Death, HealDone, Interrupt, Message, MessageType, SpellCast, Summon, UnAura, Unit};
use crate::modules::live_data_processor::material::{ActiveMapVec, Participant, WoWWOTLKParser};
use crate::modules::live_data_processor::tools::cbl_parser::combat_log_parser::CombatLogParser;
use crate::modules::live_data_processor::tools::cbl_parser::wow_tbc::parse_spell_args;
use crate::modules::live_data_processor::tools::cbl_parser::wow_wotlk::parse_damage;
use crate::modules::live_data_processor::tools::cbl_parser::wow_wotlk::parse_heal;
use crate::modules::live_data_processor::tools::cbl_parser::wow_wotlk::parse_miss;
use crate::modules::live_data_processor::tools::cbl_parser::wow_wotlk::parse_unit;

impl CombatLogParser for WoWWOTLKParser {
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
                let attacker = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let victim = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let (hit_mask, blocked, damage_component) = parse_miss(&message_args[7..])?;
                self.collect_participant(&attacker, message_args[2], event_ts);
                self.collect_participant(&victim, message_args[5], event_ts);
                self.collect_active_map(data, &attacker, event_ts);
                self.collect_active_map(data, &victim, event_ts);
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
                let attacker = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let victim = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let spell_id = parse_spell_args(&message_args[7..10])?;
                let (hit_mask, blocked, damage_component) = parse_damage(&message_args[10..])?;
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
                        blocked,
                        damage_over_time: false, // message_args[0] == "SPELL_PERIODIC_DAMAGE",
                        damage_components: vec![damage_component],
                    }),
                ]
            },
            "SPELL_MISSED" | "SPELL_PERIODIC_MISSED" | "RANGE_MISSED" | "DAMAGE_SHIELD_MISSED" => {
                let attacker = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let victim = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let spell_id = parse_spell_args(&message_args[7..10])?;
                let (hit_mask, blocked, damage_component) = parse_miss(&message_args[10..])?;
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
                        blocked,
                        damage_over_time: false, // message_args[0] == "SPELL_PERIODIC_MISSED",
                        damage_components: damage_component.map(|comp| vec![comp]).unwrap_or_else(Vec::new),
                    }),
                ]
            },
            "SPELL_HEAL" | "SPELL_PERIODIC_HEAL" => {
                let caster = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let target = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                let spell_id = parse_spell_args(&message_args[7..10])?;
                let (amount, overhealing, absorb, is_crit) = parse_heal(&message_args[10..])?;
                self.collect_participant(&caster, message_args[2], event_ts);
                self.collect_participant(&target, message_args[5], event_ts);
                self.collect_participant_class(&caster, spell_id);
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
                        absorb,
                        hit_mask: if is_crit { HitType::Crit as u32 } else { HitType::Hit as u32 },
                    }),
                ]
            },
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
            },
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
            },
            "SPELL_SUMMON" => {
                let owner = parse_unit(&message_args[1..4]).unwrap_or_else(Unit::default);
                let unit = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                vec![MessageType::Summon(Summon { owner, unit })]
            },
            "UNIT_DIED" | "UNIT_DESTROYED" => {
                let victim = parse_unit(&message_args[4..7]).unwrap_or_else(Unit::default);
                vec![MessageType::Death(Death { cause: None, victim })]
            },
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
            },
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
            },
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
            },
            // TODO: Use more events
            // https://wow.gamepedia.com/index.php?title=COMBAT_LOG_EVENT&oldid=2561876
            _ => return None,
        })
    }

    fn do_message_post_processing(&mut self, _data: &Data, _messages: &mut Vec<Message>) {
        // Do nothing
    }

    fn get_involved_server(&self) -> Option<Vec<(u32, String, String)>> {
        None
    }

    fn get_involved_character_builds(&self) -> Vec<(Option<u32>, CharacterDto)> {
        self.participants
            .iter()
            .filter(|(_, participant)| participant.is_player)
            .map(|(_, participant)| {
                (
                    None,
                    CharacterDto {
                        server_uid: participant.id,
                        character_history: Some(CharacterHistoryDto {
                            character_info: CharacterInfoDto {
                                gear: Default::default(),
                                hero_class_id: participant.hero_class_id.unwrap_or(12),
                                level: 80,
                                gender: false,
                                profession1: None,
                                profession2: None,
                                talent_specialization: None,
                                race_id: 1,
                            },
                            character_name: participant.name.clone(),
                            character_guild: None,
                            character_title: None,
                            profession_skill_points1: None,
                            profession_skill_points2: None,
                            facial: None,
                            arena_teams: vec![],
                        }),
                    },
                )
            })
            .collect()
    }

    fn get_participants(&self) -> Vec<Participant> {
        self.participants.iter().map(|(_, participant)| participant).cloned().collect()
    }

    fn get_active_maps(&self) -> ActiveMapVec {
        self.active_map.iter().map(|(_, active_map)| active_map.clone()).collect()
    }

    fn get_npc_appearance_offset(&self, entry: u32) -> Option<i64> {
        Some(match entry {
            // Kel'Thuzad
            15990 => -228000,
            // Wyrmrest Skytalon
            32535 => -30000,
            // VX-001
            33651 => -50000,
            // Yogg-Saron Npcs
            33966 | 33983 | 33985 | 33988 | 33990 | 33288 => -55000,
            _ => return None,
        })
    }

    fn get_npc_timeout(&self, entry: u32) -> Option<u64> {
        Some(match entry {
            // Kel'Thuzad
            15990 => 228000,
            // Malygos
            28859 => 180000,
            // Yogg-Saron
            33288 => 55000,
            _ => return None,
        })
    }

    fn get_death_implied_npc_combat_state_and_offset(&self, entry: u32) -> Option<Vec<(u32, i64)>> {
        Some(match entry {
            15929 | 15930 => vec![(15928, -1000)],
            _ => return None,
        })
    }

    fn get_in_combat_implied_npc_combat(&self, entry: u32) -> Option<Vec<u32>> {
        Some(match entry {
            // Yogg-Saron Npcs
            33966 | 33983 | 33985 | 33988 | 33990 => vec![33288],
            _ => return None,
        })
    }

    fn get_expansion_id(&self) -> u8 {
        3
    }

    fn get_server_id(&self) -> Option<u32> {
        Some(self.server_id)
    }
}
