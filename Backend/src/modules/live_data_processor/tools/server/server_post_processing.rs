use crate::modules::data::tools::RetrieveNPC;
use crate::modules::data::Data;
use crate::modules::live_data_processor::domain_value::{Creature, Event, EventType, Unit, UnitInstance};
use crate::modules::live_data_processor::material::Server;
use crate::params;
use crate::util::database::{Execute, Select};
use std::collections::HashMap;
use std::io::Write;

impl Server {
    pub fn perform_post_processing(&mut self, db_main: &mut (impl Execute + Select), now: u64, data: &Data) {
        self.save_current_event_id_and_end_ts(db_main);
        self.extract_attempts(db_main, data);
        // TODO: Extract Ranking
        // TODO: Extract Loot?
        self.save_committed_events_to_disk(now);
    }

    fn extract_attempts(&mut self, db_main: &mut impl Execute, data: &Data) {
        for (instance_id, committed_events) in self.committed_events.iter() {
            if let Some(UnitInstance { instance_meta_id, .. }) = self.active_instances.get(&instance_id) {
                if !self.active_attempts.contains_key(instance_id) {
                    self.active_attempts.insert(*instance_id, HashMap::with_capacity(1));
                }
                let active_attempts = self.active_attempts.get_mut(instance_id).unwrap();
                for event in committed_events.iter() {
                    if let Unit::Creature(Creature { creature_id, entry, owner: _ }) = event.subject {
                        if let Some(npc) = data.get_npc(self.expansion_id, entry) {
                            if !npc.is_boss {
                                continue;
                            }

                            match event.event {
                                EventType::CombatState { in_combat } => {
                                    // TODO: Logic non standard single bosses, i.e. group or vehicle bosses
                                    if in_combat {
                                        // We missed the going into combat event (Restart of Backend?)
                                        if let Some(attempt) = active_attempts.get(&creature_id) {
                                            commit_attempt(db_main, *instance_meta_id, creature_id, attempt.1, event.timestamp, attempt.0);
                                            active_attempts.remove(&creature_id);
                                        }
                                        active_attempts.insert(creature_id, (false, event.timestamp));
                                    } else {
                                        if let Some(attempt) = active_attempts.get(&creature_id) {
                                            commit_attempt(db_main, *instance_meta_id, creature_id, attempt.1, event.timestamp, attempt.0);
                                        }
                                        active_attempts.remove(&creature_id);
                                    }
                                },
                                EventType::Death { murder: _ } => {
                                    if let Some(attempt) = active_attempts.get_mut(&creature_id) {
                                        attempt.0 = true;
                                    } else {
                                        // Instakill?
                                        commit_attempt(db_main, *instance_meta_id, creature_id, event.timestamp, event.timestamp, true);
                                    }
                                },
                                _ => continue,
                            };
                        }
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

fn commit_attempt(db_main: &mut impl Execute, instance_meta_id: u32, creature_id: u64, attempt_start: u64, attempt_end: u64, is_kill: bool) {
    db_main.execute_wparams(
        "INSERT INTO `instance_attempt` (`instance_meta_id`, `creature_id`, `start_ts`, `end_ts`, `is_kill`) VALUES (:instance_meta_id, :creature_id, :start_ts, :end_ts, :is_kill)",
        params!("instance_meta_id" => instance_meta_id, "creature_id" => creature_id, "start_ts" => attempt_start, "end_ts" => attempt_end, "is_kill" => is_kill),
    );
}
