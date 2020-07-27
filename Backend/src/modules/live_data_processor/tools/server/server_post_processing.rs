use crate::modules::data::tools::RetrieveNPC;
use crate::modules::data::Data;
use crate::modules::live_data_processor::domain_value::{Creature, Event, EventType, Player, Unit, UnitInstance};
use crate::modules::live_data_processor::material::{Attempt, Server};
use crate::params;
use crate::util::database::{Execute, Select};
use std::collections::HashMap;
use std::io::Write;

impl Server {
    pub fn perform_post_processing(&mut self, db_main: &mut (impl Execute + Select), now: u64, data: &Data) {
        self.save_current_event_id_and_end_ts(db_main);
        self.extract_attempts_and_collect_ranking(db_main, data);
        // TODO: Extract Loot?
        self.save_committed_events_to_disk(now);
    }

    fn extract_attempts_and_collect_ranking(&mut self, db_main: &mut (impl Execute + Select), data: &Data) {
        for (instance_id, committed_events) in self.committed_events.iter() {
            if let Some(UnitInstance { instance_meta_id, .. }) = self.active_instances.get(&instance_id) {
                if !self.active_attempts.contains_key(instance_id) {
                    self.active_attempts.insert(*instance_id, HashMap::with_capacity(1));
                }
                let active_attempts = self.active_attempts.get_mut(instance_id).unwrap();
                for event in committed_events.iter() {
                    match &event.subject {
                        Unit::Creature(Creature { creature_id, entry, owner: _ }) => {
                            if let Some(npc) = data.get_npc(self.expansion_id, *entry) {
                                if !npc.is_boss {
                                    continue;
                                }

                                match event.event {
                                    EventType::CombatState { in_combat } => {
                                        // TODO: Logic non standard single bosses, i.e. group or vehicle bosses
                                        if in_combat {
                                            // We missed the going into combat event (Restart of Backend?)
                                            if let Some(mut attempt) = active_attempts.remove(&creature_id) {
                                                attempt.end_ts = event.timestamp;
                                                commit_attempt(db_main, *instance_meta_id, attempt);
                                            }
                                            active_attempts.insert(*creature_id, Attempt::new(*creature_id, *entry, event.timestamp));
                                        } else if let Some(mut attempt) = active_attempts.remove(&creature_id) {
                                            attempt.end_ts = event.timestamp;
                                            commit_attempt(db_main, *instance_meta_id, attempt);
                                        }
                                    },
                                    EventType::Death { murder: _ } => {
                                        if let Some(attempt) = active_attempts.get_mut(&creature_id) {
                                            attempt.is_kill = true;
                                        } else {
                                            // Instakill?
                                            commit_attempt(
                                                db_main,
                                                *instance_meta_id,
                                                Attempt {
                                                    is_kill: false,
                                                    creature_id: *creature_id,
                                                    npc_id: *entry,
                                                    start_ts: event.timestamp,
                                                    end_ts: event.timestamp,
                                                    ranking_damage: HashMap::new(),
                                                    ranking_heal: HashMap::new(),
                                                    ranking_threat: HashMap::new(),
                                                },
                                            );
                                        }
                                    },
                                    _ => continue,
                                };
                            }
                        },
                        Unit::Player(Player { character_id, .. }) => {
                            match &event.event {
                                EventType::SpellDamage { damage, .. } | EventType::MeleeDamage(damage) => {
                                    if let Unit::Creature(Creature { creature_id, .. }) = damage.victim {
                                        if let Some(attempt) = active_attempts.get_mut(&creature_id) {
                                            if let Some(player_damage) = attempt.ranking_damage.get_mut(character_id) {
                                                *player_damage += damage.damage;
                                            } else {
                                                attempt.ranking_damage.insert(*character_id, damage.damage);
                                            }
                                        }
                                    }
                                },
                                EventType::Heal { heal, .. } => {
                                    // TODO: We can't really tell, who this healer is in combat with
                                    // For now attribute heal to every attempt, though its just one in 99.9% of the cases anyway.
                                    // And in some cases its not even wrong, e.g. BWL where the dragons are cleaved
                                    for (_, attempt) in active_attempts.iter_mut() {
                                        if let Some(player_heal) = attempt.ranking_heal.get_mut(character_id) {
                                            *player_heal += heal.effective;
                                        } else {
                                            attempt.ranking_heal.insert(*character_id, heal.effective);
                                        }
                                    }
                                },
                                EventType::Threat { threat, .. } => {
                                    if let Unit::Creature(Creature { creature_id, .. }) = threat.threatened {
                                        if let Some(attempt) = active_attempts.get_mut(&creature_id) {
                                            if let Some(player_threat) = attempt.ranking_threat.get_mut(character_id) {
                                                *player_threat += threat.amount;
                                            } else {
                                                attempt.ranking_threat.insert(*character_id, threat.amount);
                                            }
                                        }
                                    }
                                },
                                _ => continue,
                            }
                        },
                    }
                }
            }
        }
    }

    fn save_current_event_id_and_end_ts(&self, db_main: &mut impl Execute) {
        for (instance_id, current_event_id) in self.committed_events_count.iter() {
            if let Some(UnitInstance { instance_meta_id, .. }) = self.active_instances.get(&instance_id) {
                if let Some(committed_events) = self.committed_events.get(&instance_id) {
                    if let Some(last_entry) = committed_events.last() {
                        db_main.execute_wparams(
                            "UPDATE instance_meta SET last_event_id=:current_event_id, end_ts=:end_ts WHERE instance_meta_id=:instance_meta_id",
                            params!("current_event_id" => *current_event_id, "end_ts" => last_entry.timestamp, "instance_meta_id" => instance_meta_id),
                        );
                    }
                }
            }
        }
    }

    fn save_committed_events_to_disk(&mut self, now: u64) {
        let storage_path = std::env::var("INSTANCE_STORAGE_PATH").expect("storage path must be set!");
        let mut open_options = std::fs::File::with_options();
        open_options.append(true);
        open_options.create(true);

        for (instance_id, active_instance) in self.active_instances.iter() {
            if let Some(committable_events) = self.committed_events.get_mut(&instance_id) {
                let _result = std::fs::create_dir_all(format!("{}/{}/{}", storage_path, self.server_id, active_instance.instance_meta_id));

                // Find first event that is committable from the back
                if let Some(extraction_index) = committable_events.iter().rposition(|event| event.timestamp + 5000 < now) {
                    let mut drained_events = committable_events.drain(..(extraction_index + 1)).collect::<Vec<Event>>();
                    drained_events.sort_by(|left, right| {
                        let by_event_type = left.event.to_u8().cmp(&right.event.to_u8());
                        if let std::cmp::Ordering::Equal = by_event_type {
                            return left.timestamp.cmp(&right.timestamp);
                        }
                        by_event_type
                    });

                    let mut last_opened_event_type_index = 0;
                    let mut opened_file = open_options.open(format!("{}/{}/{}/{}", storage_path, self.server_id, active_instance.instance_meta_id, 0));
                    for event in drained_events {
                        if event.event.to_u8() != last_opened_event_type_index {
                            last_opened_event_type_index = event.event.to_u8();
                            opened_file = opened_file.and_then(|file| {
                                drop(file);
                                open_options.open(format!("{}/{}/{}/{}", storage_path, self.server_id, active_instance.instance_meta_id, event.event.to_u8()))
                            })
                        }
                        if let Ok(file) = &mut opened_file {
                            if let Ok(json) = serde_json::to_string(&event) {
                                let _ = file.write(json.as_bytes());
                                let _ = file.write(&[10]);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn commit_attempt(db_main: &mut (impl Execute + Select), instance_meta_id: u32, attempt: Attempt) {
    let params = params!("instance_meta_id" => instance_meta_id, "creature_id" => attempt.creature_id, "start_ts" => attempt.start_ts, "end_ts" => attempt.end_ts, "is_kill" => attempt.is_kill);
    db_main.execute_wparams(
        "INSERT INTO `instance_attempt` (`instance_meta_id`, `creature_id`, `start_ts`, `end_ts`, `is_kill`) VALUES (:instance_meta_id, :creature_id, :start_ts, :end_ts, :is_kill)",
        params.clone(),
    );

    if !attempt.is_kill {
        return;
    }

    if let Some(attempt_id) = db_main.select_wparams_value(
        "SELECT id FROM `instance_attempt` WHERE instance_meta_id=:instance_meta_id AND creature_id=:creature_id AND start_ts=:start_ts AND end_ts=:end_ts AND is_kill=:is_kill",
        |mut row| row.take::<u32, usize>(0),
        params,
    ) {
        let attempt_clone = attempt.clone();
        db_main.execute_batch_wparams(
            "INSERT INTO `instance_attempt_damage` (`character_id`, `npc_id`, `attempt_id`, `damage`) VALUES (:character_id, :npc_id, :attempt_id, :damage)",
            attempt.ranking_damage.clone().into_iter().collect(),
            move |(character_id, damage)| {
                params! {
                    "character_id" => character_id,
                    "npc_id" => attempt_clone.npc_id,
                    "attempt_id" => attempt_id,
                    "damage" => damage
                }
            },
        );

        let attempt_clone = attempt.clone();
        db_main.execute_batch_wparams(
            "INSERT INTO `instance_attempt_heal` (`character_id`, `npc_id`, `attempt_id`, `heal`) VALUES (:character_id, :npc_id, :attempt_id, :heal)",
            attempt.ranking_heal.clone().into_iter().collect(),
            move |(character_id, heal)| {
                params! {
                    "character_id" => character_id,
                    "npc_id" => attempt_clone.npc_id,
                    "attempt_id" => attempt_id,
                    "heal" => heal
                }
            },
        );

        db_main.execute_batch_wparams(
            "INSERT INTO `instance_attempt_threat` (`character_id`, `npc_id`, `attempt_id`, `threat`) VALUES (:character_id, :npc_id, :attempt_id, :threat)",
            attempt.ranking_threat.clone().into_iter().collect(),
            move |(character_id, threat)| {
                params! {
                    "character_id" => character_id,
                    "npc_id" => attempt.npc_id,
                    "attempt_id" => attempt_id,
                    "threat" => threat
                }
            },
        );
    }
}
