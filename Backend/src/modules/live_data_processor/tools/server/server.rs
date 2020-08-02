use crate::modules::armory::tools::GetArenaTeam;
use crate::modules::armory::Armory;
use crate::modules::data::Data;
use crate::modules::live_data_processor::domain_value::{AuraApplication, Event, EventParseFailureAction, EventType, HitType, Mitigation, Position, Power, PowerType, School, Unit, UnitInstance};
use crate::modules::live_data_processor::dto::{CombatState, Death, Loot, Summon};
use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Message, MessageType};
use crate::modules::live_data_processor::material::Server;
use crate::modules::live_data_processor::tools::server::{try_parse_dispel, try_parse_interrupt, try_parse_spell_steal};
use crate::modules::live_data_processor::tools::MapUnit;
use crate::modules::live_data_processor::{domain_value, dto};
use crate::params;
use crate::util::database::{Execute, Select};
use std::collections::{BTreeSet, VecDeque};

impl Server {
    pub fn parse_events(&mut self, db_main: &mut (impl Select + Execute), armory: &Armory, data: &Data, messages: Vec<Message>) -> Result<(), LiveDataProcessorFailure> {
        let mut next_reset = 0;
        let post_processing_interval = 5 * 1000;
        let mut next_post_processing = 0;

        for msg in messages {
            self.extract_meta_information(db_main, armory, &msg);
            self.test_for_committable_events(db_main, armory);
            self.cleanup(msg.timestamp);
            if next_reset < msg.timestamp || next_reset == u64::MAX {
                next_reset = self.reset_instances(db_main, msg.timestamp);
            }
            if next_post_processing < msg.timestamp {
                self.perform_post_processing(db_main, msg.timestamp, data);
                next_post_processing = msg.timestamp + post_processing_interval;
            }
            self.push_non_committed_event(msg);
        }
        Ok(())
    }

    fn push_non_committed_event(&mut self, message: Message) {
        if let Some(unit_dto) = message.message_type.extract_subject() {
            if let Some(event) = self.non_committed_events.get_mut(&unit_dto.unit_id) {
                if self.subject_prepend_mode_set.contains(&unit_dto.unit_id) {
                    event.push_front(message);
                } else {
                    event.push_back(message);
                }
            } else {
                self.non_committed_events.insert(unit_dto.unit_id, VecDeque::from(vec![message]));
            }
            self.subject_prepend_mode_set.remove(&unit_dto.unit_id);
        }
    }

    fn test_for_committable_events(&mut self, db_main: &mut (impl Select + Execute), armory: &Armory) {
        let mut remove_first_non_committed_event = Vec::new();
        for (subject_id, non_committed_event) in self.non_committed_events.iter() {
            match self.commit_event(db_main, armory, non_committed_event) {
                Ok(mut committable_event) => {
                    // For all except Spell we want to only remove the first event
                    remove_first_non_committed_event.push(*subject_id);

                    if let Some(unit_instance_id) = self.unit_instance_id.get(subject_id) {
                        if let Some(instance_events) = self.committed_events.get_mut(unit_instance_id) {
                            let committed_event_count = self.committed_events_count.get_mut(unit_instance_id).expect("Should exist if this exists");
                            committable_event.id = *committed_event_count;
                            *committed_event_count += 1;
                            instance_events.push(committable_event);
                        } else {
                            committable_event.id = 1;
                            self.committed_events.insert(*unit_instance_id, vec![committable_event]);
                            self.committed_events_count.insert(*unit_instance_id, 2);
                        }
                    }
                    // Else discard I guess
                },
                Err(EventParseFailureAction::DiscardFirst) => {
                    remove_first_non_committed_event.push(*subject_id);
                },
                Err(EventParseFailureAction::PrependNext) => {
                    self.subject_prepend_mode_set.insert(*subject_id);
                },
                Err(EventParseFailureAction::Wait) => {},
            };
        }

        for subject_id in remove_first_non_committed_event {
            self.non_committed_events.get_mut(&subject_id).expect("subject id should exist").pop_front();
            self.subject_prepend_mode_set.remove(&subject_id);
            if self.non_committed_events.get(&subject_id).expect("subject id should exist").is_empty() {
                self.non_committed_events.remove(&subject_id);
            }
        }
    }

    fn cleanup(&mut self, current_timestamp: u64) {
        for subject_id in self
            .non_committed_events
            .iter()
            .filter(|(_subject_id, event)| event.front().expect("Should be initialized with at least one element").timestamp + 500 < current_timestamp)
            .map(|(subject_id, _event)| *subject_id)
            .collect::<Vec<u64>>()
        {
            self.non_committed_events.remove(&subject_id);
        }
    }

    // So based on the next event for the current users in the system
    // we are going to decide whether or not to commit it.
    fn commit_event(&self, db_main: &mut (impl Select + Execute), armory: &Armory, non_committed_event: &VecDeque<Message>) -> Result<Event, EventParseFailureAction> {
        let first_message = non_committed_event.front().expect("non_committed_event contains at least one entry");
        match &first_message.message_type {
            // Events that are just of size 1
            MessageType::CombatState(CombatState { unit: unit_dto, in_combat }) => Ok(Event::new(
                first_message.timestamp,
                unit_dto.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                EventType::CombatState { in_combat: *in_combat },
            )),
            MessageType::Loot(Loot { unit: unit_dto, item_id, count }) => Ok(Event::new(
                first_message.timestamp,
                unit_dto.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                EventType::Loot { item_id: *item_id, amount: *count },
            )),
            MessageType::Position(position) => Ok(Event::new(
                first_message.timestamp,
                position.unit.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                EventType::Position(Position {
                    x: position.x,
                    y: position.y,
                    z: position.z,
                    orientation: position.orientation,
                }),
            )),
            MessageType::Power(power) => Ok(Event::new(
                first_message.timestamp,
                power.unit.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                EventType::Power(Power {
                    power_type: PowerType::from_u8(power.power_type).ok_or_else(|| EventParseFailureAction::DiscardFirst)?,
                    max_power: power.max_power,
                    current_power: power.current_power,
                }),
            )),
            MessageType::AuraApplication(aura_application) => Ok(Event::new(
                first_message.timestamp,
                aura_application.target.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                EventType::AuraApplication(AuraApplication {
                    caster: aura_application.caster.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                    stack_amount: aura_application.stack_amount,
                    spell_id: aura_application.spell_id,
                }),
            )),
            MessageType::Death(Death { cause, victim }) => Ok(Event::new(
                first_message.timestamp,
                victim.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                EventType::Death {
                    murder: cause.as_ref().and_then(|cause| cause.to_unit(db_main, &armory, self.server_id, &self.summons).ok()),
                },
            )),
            MessageType::Event(event_dto) => {
                if event_dto.event_type == 0 {
                    if let Ok(creature @ domain_value::Unit::Creature(_)) = event_dto.unit.to_unit(db_main, armory, self.server_id, &self.summons) {
                        // TODO: Is the creature really the unit that we want to return here?
                        return Ok(Event::new(first_message.timestamp, creature, EventType::ThreatWipe));
                    }
                }
                Err(EventParseFailureAction::DiscardFirst)
            },
            MessageType::Summon(summon) => {
                let summoner = summon.owner.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                let summoned = summon.unit.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                Ok(Event::new(first_message.timestamp, summoner, EventType::Summon { summoned }))
            },
            MessageType::SpellCast(spell_cast) => {
                let subject = spell_cast.caster.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                Ok(Event::new(
                    first_message.timestamp,
                    subject,
                    EventType::SpellCast(domain_value::SpellCast {
                        victim: spell_cast.target.as_ref().and_then(|victim| victim.to_unit(db_main, armory, self.server_id, &self.summons).ok()),
                        hit_type: HitType::from_u8(spell_cast.hit_type),
                        spell_id: spell_cast.spell_id,
                    }),
                ))
            },
            MessageType::MeleeDamage(melee_damage) => {
                let subject = melee_damage.attacker.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                let victim = melee_damage.victim.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                let mut mitigation = Vec::with_capacity(3);
                if melee_damage.blocked > 0 {
                    mitigation.push(Mitigation::Block(melee_damage.blocked));
                }
                if melee_damage.absorbed > 0 {
                    mitigation.push(Mitigation::Absorb(melee_damage.absorbed));
                }
                if melee_damage.resisted_or_glanced > 0 {
                    mitigation.push(Mitigation::Glance(melee_damage.resisted_or_glanced));
                }
                Ok(Event::new(
                    first_message.timestamp,
                    subject,
                    EventType::MeleeDamage(domain_value::Damage {
                        school: School::from_u8(melee_damage.school),
                        damage: melee_damage.damage,
                        mitigation,
                        victim,
                        hit_type: HitType::from_u8(melee_damage.hit_type.expect("Can melee damage be None?")),
                    }),
                ))
            },
            MessageType::SpellDamage(spell_damage) => {
                let subject = spell_damage.attacker.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                let spell_cast_id = self.find_matching_spell_cast(spell_damage.spell_id.unwrap(), spell_damage.attacker.unit_id, &subject)?;
                let victim = spell_damage.victim.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                let mut mitigation = Vec::with_capacity(3);
                if spell_damage.blocked > 0 {
                    mitigation.push(Mitigation::Block(spell_damage.blocked));
                }
                if spell_damage.absorbed > 0 {
                    mitigation.push(Mitigation::Absorb(spell_damage.absorbed));
                }
                if spell_damage.resisted_or_glanced > 0 {
                    mitigation.push(Mitigation::Resist(spell_damage.resisted_or_glanced));
                }
                Ok(Event::new(
                    first_message.timestamp,
                    subject,
                    EventType::SpellDamage {
                        spell_cast_id,
                        damage: domain_value::Damage {
                            school: School::from_u8(spell_damage.school),
                            damage: spell_damage.damage,
                            mitigation,
                            victim,
                            // TODO: How to tell its a crit?
                            hit_type: spell_damage.hit_type.map_or(HitType::Hit, HitType::from_u8),
                        },
                    },
                ))
            },
            MessageType::Heal(heal_done) => {
                let subject = heal_done.caster.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                let spell_cast_id = self.find_matching_spell_cast(heal_done.spell_id, heal_done.caster.unit_id, &subject)?;
                let target = heal_done.target.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                Ok(Event::new(
                    first_message.timestamp,
                    subject,
                    EventType::Heal {
                        spell_cast_id,
                        heal: domain_value::Heal {
                            total: heal_done.total_heal,
                            effective: heal_done.effective_heal,
                            mitigation: if heal_done.absorb > 0 { vec![Mitigation::Absorb(heal_done.absorb)] } else { Vec::new() },
                            target,
                        },
                    },
                ))
            },
            // A SpellCast, Damage done and heal done can cause threat
            // Note: That the threatened unit can be a third unit in the case of a beneficial spell
            MessageType::Threat(threat) => {
                let subject = threat.threater.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                let threatened = threat.threatened.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                let cause_event_id = self.find_matching_threat_cause(threat.spell_id, threat.threater.unit_id, &subject, &threatened)?;
                Ok(Event::new(
                    first_message.timestamp,
                    subject,
                    EventType::Threat {
                        cause_event_id,
                        threat: domain_value::Threat { threatened, amount: threat.amount },
                    },
                ))
            },
            // Find Event that caused this interrupt, else wait or discard
            MessageType::Interrupt(interrupt) => {
                // If we dont find any committable events for this interrupt, we need to discard
                if let Some(unit_instance_id) = self.unit_instance_id.get(&interrupt.target.unit_id) {
                    if let Some(committed_events) = self.committed_events.get(unit_instance_id) {
                        let subject = interrupt.target.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                        return match try_parse_interrupt(&interrupt, committed_events, first_message.timestamp, &subject) {
                            Ok((cause_event_id, interrupted_spell_id)) => Ok(Event::new(first_message.timestamp, subject, EventType::Interrupt { cause_event_id, interrupted_spell_id })),
                            Err(err) => Err(err),
                        };
                    }
                }
                Err(EventParseFailureAction::PrependNext)
            },
            // Find Event that caused this dispel, else wait or discard
            MessageType::Dispel(dispel) => {
                // If we dont find any committable events for this interrupt, we need to discard
                if let Some(unit_instance_id) = self.unit_instance_id.get(&dispel.un_aura_caster.unit_id) {
                    if let Some(committed_events) = self.committed_events.get(unit_instance_id) {
                        let subject = dispel.un_aura_caster.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                        return match try_parse_dispel(db_main, &dispel, committed_events, first_message.timestamp, armory, self.server_id, &self.summons) {
                            Ok((cause_event_id, target_event_ids)) => Ok(Event::new(first_message.timestamp, subject, EventType::Dispel { cause_event_id, target_event_ids })),
                            Err(err) => Err(err),
                        };
                    }
                }
                Err(EventParseFailureAction::PrependNext)
            },
            // Find Event that caused this spell steal, else wait or discard
            MessageType::SpellSteal(spell_steal) => {
                // If we dont find any committable events for this interrupt, we need to discard
                if let Some(unit_instance_id) = self.unit_instance_id.get(&spell_steal.un_aura_caster.unit_id) {
                    if let Some(committed_events) = self.committed_events.get(unit_instance_id) {
                        let subject = spell_steal.aura_caster.to_unit(db_main, armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                        return match try_parse_spell_steal(db_main, &spell_steal, committed_events, first_message.timestamp, armory, self.server_id, &self.summons) {
                            Ok((cause_event_id, target_event_id)) => Ok(Event::new(first_message.timestamp, subject, EventType::SpellSteal { cause_event_id, target_event_id })),
                            Err(err) => Err(err),
                        };
                    }
                }
                Err(EventParseFailureAction::PrependNext)
            },
            _ => Err(EventParseFailureAction::DiscardFirst),
        }
    }

    fn extract_meta_information(&mut self, db_main: &mut (impl Select + Execute), armory: &Armory, message: &Message) {
        match &message.message_type {
            MessageType::Summon(Summon { owner, unit }) => {
                self.summons.insert(owner.unit_id, unit.unit_id);
            },
            MessageType::Position(dto::Position { map_id, instance_id, map_difficulty, unit, .. }) => {
                // TODO: How to handle Worldbosses in Vanilla / TBC
                // These are raids
                if let 249 | 309 | 409 | 469 | 509 | 531 | 532 | 533 | 534 | 544 | 548 | 550 | 564 | 565 | 568 | 580 | 603 | 615 | 616 | 624 | 631 | 649 | 724 = *map_id {
                    if let Some(instance_meta_id) = self.create_instance_meta(db_main, message.timestamp, *instance_id, *map_id) {
                        // Vanilla does usually not set difficulty for raids correctly
                        // Nor does TBC
                        let map_difficulty = match *map_id {
                            249 | 409 | 469 => 9,                         // 40 man
                            309 | 509 | 531 => 148,                       // 20 man
                            532 | 568 => 3,                               // 10 man
                            534 | 544 | 548 | 550 | 564 | 580 | 565 => 4, // 25 man
                            533 => {
                                if *map_difficulty == 3 || *map_difficulty == 4 {
                                    *map_difficulty
                                } else {
                                    9
                                }
                            }, // Naxx
                            _ => *map_difficulty,
                        };

                        db_main.execute_wparams(
                            "INSERT INTO instance_raid (`instance_meta_id`, `map_difficulty`) VALUES (:instance_meta_id, :map_difficulty)",
                            params!("instance_meta_id" => instance_meta_id, "map_difficulty" => map_difficulty),
                        );
                    }
                }
                self.unit_instance_id.insert(unit.unit_id, *instance_id);

                // Insert participants
                if let Some(UnitInstance { instance_meta_id, .. }) = self.active_instances.get(instance_id) {
                    if !self.instance_participants.contains_key(instance_meta_id) {
                        self.instance_participants.insert(*instance_meta_id, BTreeSet::new());
                    }
                    if let Ok(domain_value::Unit::Player(player)) = unit.to_unit(db_main, armory, self.server_id, &self.summons) {
                        let participants = self.instance_participants.get_mut(instance_meta_id).unwrap();
                        if !participants.contains(&player.character_id)
                            && db_main.execute_wparams(
                                "INSERT INTO instance_participants (`instance_meta_id`, `character_id`) VALUES (:instance_meta_id, :character_id)",
                                params!("instance_meta_id" => instance_meta_id, "character_id" => player.character_id),
                            )
                        {
                            participants.insert(player.character_id);
                        }
                    }
                }
            },
            MessageType::InstancePvPStartUnratedArena(dto::InstanceStart { map_id, instance_id }) => {
                if let Some(instance_meta_id) = self.create_instance_meta(db_main, message.timestamp, *instance_id, *map_id) {
                    db_main.execute_wparams("INSERT INTO instance_skirmish (`instance_meta_id`) VALUES (:instance_meta_id)", params!("instance_meta_id" => instance_meta_id));
                }
            },
            MessageType::InstancePvPStartRatedArena(dto::InstanceStartRatedArena { map_id, instance_id, team_id1, team_id2 }) => {
                if let Some(instance_meta_id) = self.create_instance_meta(db_main, message.timestamp, *instance_id, *map_id) {
                    if let Some(team1) = armory.get_arena_team_by_uid(db_main, self.server_id, *team_id1) {
                        if let Some(team2) = armory.get_arena_team_by_uid(db_main, self.server_id, *team_id2) {
                            db_main.execute_wparams(
                                "INSERT INTO instance_rated_arena (`instance_meta_id`, `team_id1`, `team_id2`) VALUES (:instance_meta_id, :team_id1, :team_id2)",
                                params!(
                                "instance_meta_id" => instance_meta_id,
                                "team_id1" => team1.id,
                                "team_id2" => team2.id
                                ),
                            );
                        }
                    }
                }
            },
            MessageType::InstancePvPStartBattleground(dto::InstanceStart { map_id, instance_id }) => {
                if let Some(instance_meta_id) = self.create_instance_meta(db_main, message.timestamp, *instance_id, *map_id) {
                    db_main.execute_wparams("INSERT INTO instance_battleground (`instance_meta_id`) VALUES (:instance_meta_id)", params!("instance_meta_id" => instance_meta_id));
                }
            },
            MessageType::InstancePvPEndBattleground(dto::InstanceBattleground {
                instance_id,
                winner,
                score_alliance,
                score_horde,
                ..
            }) => {
                if let Some(UnitInstance { instance_meta_id, .. }) = self.active_instances.get(instance_id) {
                    self.finalize_instance_meta(db_main, message.timestamp, *instance_meta_id);
                    db_main.execute_wparams(
                        "UPDATE instance_battleground SET `winner`=:winner, `score_alliance`=:score_alliance, `score_horde`=:score_horde WHERE instance_meta_id=:instance_meta_id",
                        params!(
                            "instance_meta_id" => *instance_meta_id,
                            "winner" => *winner,
                            "score_alliance" => *score_alliance,
                            "score_horde" => *score_horde
                        ),
                    );
                    self.instance_participants.remove(instance_meta_id);
                    self.active_instances.remove(instance_id);
                    self.active_attempts.remove(instance_id);
                }
            },
            MessageType::InstancePvPEndRatedArena(dto::InstanceArena {
                instance_id,
                winner,
                team_change1,
                team_change2,
                ..
            }) => {
                if let Some(UnitInstance { instance_meta_id, .. }) = self.active_instances.get(instance_id) {
                    self.finalize_instance_meta(db_main, message.timestamp, *instance_meta_id);
                    db_main.execute_wparams(
                        "UPDATE instance_rated_arena SET `winner`=:winner, `team_change1`=:team_change1, `team_change2`=:team_change2 WHERE instance_meta_id=:instance_meta_id",
                        params!(
                            "instance_meta_id" => *instance_meta_id,
                            "winner" => *winner,
                            "team_change1" => *team_change1,
                            "team_change2" => *team_change2
                        ),
                    );
                    self.instance_participants.remove(instance_meta_id);
                    self.active_instances.remove(instance_id);
                    self.active_attempts.remove(instance_id);
                }
            },
            MessageType::InstancePvPEndUnratedArena(dto::InstanceUnratedArena { instance_id, winner, .. }) => {
                if let Some(UnitInstance { instance_meta_id, .. }) = self.active_instances.get(instance_id) {
                    self.finalize_instance_meta(db_main, message.timestamp, *instance_meta_id);
                    db_main.execute_wparams(
                        "UPDATE instance_skirmish `winner`=:winner WHERE instance_meta_id=:instance_meta_id",
                        params!(
                            "instance_meta_id" => *instance_meta_id,
                            "winner" => *winner
                        ),
                    );
                    self.instance_participants.remove(instance_meta_id);
                    self.active_instances.remove(instance_id);
                    self.active_attempts.remove(instance_id);
                }
            },
            MessageType::InstanceDelete { instance_id } => {
                if let Some(UnitInstance { instance_meta_id, .. }) = self.active_instances.get(instance_id) {
                    if self.finalize_instance_meta(db_main, message.timestamp, *instance_meta_id) {
                        self.instance_participants.remove(instance_meta_id);
                        self.active_instances.remove(instance_id);
                        self.active_attempts.remove(instance_id);
                    }
                }
            },
            _ => {},
        }
    }

    fn create_instance_meta(&mut self, db_main: &mut (impl Execute + Select), start_ts: u64, instance_id: u32, map_id: u32) -> Option<u32> {
        if !self.active_instances.contains_key(&instance_id) {
            // Maybe sanity check, if active instance already exists, before?
            if db_main.execute_wparams(
                "INSERT INTO instance_meta (`server_id`, `start_ts`, `instance_id`, `map_id`) VALUES (:server_id, :start_ts, :instance_id, :map_id)",
                params!(
                "server_id" => self.server_id,
                "start_ts" => start_ts,
                "instance_id" => instance_id,
                "map_id" => map_id as u16
                ),
            ) {
                let instance_meta_id = db_main
                    .select_wparams_value(
                        "SELECT id FROM instance_meta WHERE server_id=:server_id AND instance_id=:instance_id AND map_id=:map_id AND expired IS NULL",
                        |mut row| row.take::<u32, usize>(0).unwrap(),
                        params!(
                        "server_id" => self.server_id,
                        "instance_id" => instance_id,
                        "map_id" => map_id as u16
                        ),
                    )
                    .expect("Should exist and DB shouldn't have gone away");

                self.active_instances.insert(
                    instance_id,
                    UnitInstance {
                        instance_meta_id,
                        entered: start_ts,
                        map_id: map_id as u16,
                        instance_id,
                    },
                );

                return Some(instance_meta_id);
            }
        }
        None
    }

    fn finalize_instance_meta(&self, db_main: &mut impl Execute, end_ts: u64, instance_meta_id: u32) -> bool {
        db_main.execute_wparams(
            "UPDATE instance_meta SET end_ts=IF(end_ts IS NULL, :end_ts, end_ts), expired=:end_ts WHERE instance_meta_id=:instance_meta_id",
            params!(
                "end_ts" => end_ts,
                "instance_meta_id" => instance_meta_id
            ),
        )
    }

    /// Returns timestamp when the next reset is required
    pub fn reset_instances(&mut self, db_main: &mut impl Execute, now: u64) -> u64 {
        for (instance_id, instance_meta_id) in self
            .active_instances
            .iter()
            .filter(|(_, active_instance)| {
                if let Some(instance_reset) = self.instance_resets.get(&active_instance.map_id) {
                    return active_instance.entered <= instance_reset.reset_time && now > instance_reset.reset_time;
                }
                false
            })
            .map(|(instance_id, unit_instance)| (*instance_id, unit_instance.instance_meta_id))
            .collect::<Vec<(u32, u32)>>()
        {
            if self.finalize_instance_meta(db_main, now, instance_meta_id) {
                self.active_instances.remove(&instance_id);
                self.instance_participants.remove(&instance_meta_id);
                self.active_attempts.remove(&instance_id);
            }
        }
        self.instance_resets
            .iter()
            .filter(|(_, active_instance)| active_instance.reset_time >= now)
            .fold(u64::MAX, |acc, (_, active_instance)| acc.min(active_instance.reset_time))
    }

    fn find_matching_spell_cast(&self, spell_id: u32, subject_unit_id: u64, subject: &Unit) -> Result<u32, EventParseFailureAction> {
        if let Some(unit_instance_id) = self.unit_instance_id.get(&subject_unit_id) {
            if let Some(committed_events) = self.committed_events.get(unit_instance_id) {
                if let Some(event_index) = committed_events.iter().rposition(|event| {
                    if let EventType::SpellCast(spell_cast) = &event.event {
                        return spell_cast.spell_id == spell_id && event.subject == *subject;
                    }
                    false
                }) {
                    return Ok(committed_events.get(event_index).unwrap().id);
                }
            }
        }
        Err(EventParseFailureAction::PrependNext)
    }

    // If the spell_id is some, then it must be a SpellCast, SpellDamage or Heal Done
    // Else its a melee_damage event and the threatened must be the victim
    fn find_matching_threat_cause(&self, spell_id: Option<u32>, subject_unit_id: u64, subject: &Unit, threatened: &Unit) -> Result<u32, EventParseFailureAction> {
        if let Some(unit_instance_id) = self.unit_instance_id.get(&subject_unit_id) {
            if let Some(committed_events) = self.committed_events.get(unit_instance_id) {
                if let Some(event_index) = committed_events.iter().rposition(|event| {
                    if let Some(threatening_spell_id) = spell_id {
                        return event.subject == *subject
                            && match &event.event {
                                EventType::SpellCast(domain_value::SpellCast { spell_id, .. }) => *spell_id == threatening_spell_id,
                                EventType::SpellDamage { spell_cast_id, .. } | EventType::Heal { spell_cast_id, .. } => {
                                    if let Some(Event { event: EventType::SpellCast(spell_cast), .. }) = committed_events.iter().find(|inner_event| inner_event.id == *spell_cast_id) {
                                        spell_cast.spell_id == threatening_spell_id
                                    } else {
                                        false
                                    }
                                },
                                _ => false,
                            };
                    } else if let EventType::MeleeDamage(melee_damage) = &event.event {
                        return melee_damage.victim == *threatened && event.subject == *subject;
                    }
                    false
                }) {
                    return Ok(committed_events.get(event_index).unwrap().id);
                }
            }
        }
        Err(EventParseFailureAction::PrependNext)
    }
}
