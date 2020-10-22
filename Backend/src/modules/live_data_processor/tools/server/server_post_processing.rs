#![allow(clippy::if_same_then_else)]

use crate::modules::data::tools::{RetrieveEncounterNpc, RetrieveItem};
use crate::modules::data::Data;
use crate::modules::live_data_processor::domain_value::get_spell_components_total;
use crate::modules::live_data_processor::domain_value::{Creature, Event, EventType, Player, Power, PowerType, Unit, UnitInstance};
use crate::modules::live_data_processor::material::{Attempt, Server};
use crate::modules::live_data_processor::tools::LiveDataDeserializer;
use crate::params;
use crate::util::database::{Execute, Select};
use std::collections::{HashMap, VecDeque};
use std::io::Write;
use std::ops::Div;

impl Server {
    pub fn perform_post_processing(&mut self, db_main: &mut (impl Execute + Select), now: u64, data: &Data) {
        self.extract_attempts_and_collect_ranking(db_main, data, now);
        self.extract_loot(db_main, data, now);
        self.save_current_event_id_and_end_ts(db_main);
        self.save_committed_events_to_disk(now);
    }

    fn extract_loot(&self, db_main: &mut (impl Execute + Select), data: &Data, now: u64) {
        for (instance_id, committed_events) in self.committed_events.iter() {
            if let Some(UnitInstance { instance_meta_id, .. }) = self.active_instances.get(&instance_id) {
                for event in committed_events.iter() {
                    if event.timestamp + 2000 > now {
                        break;
                    }

                    if let EventType::Loot { item_id, amount } = &event.event {
                        if let Unit::Player(Player { character_id, .. }) = event.subject {
                            if let Some(item) = data.get_item(self.expansion_id, *item_id) {
                                if item.quality >= 5 {
                                    // Epic or better
                                    db_main.execute_wparams(
                                        "INSERT INTO instance_loot (`instance_meta_id`, `character_id`, `item_id`, `looted_ts`, `amount`) VALUES (:instance_meta_id, :character_id, :item_id, :looted_ts, :amount)",
                                        params!("instance_meta_id" => *instance_meta_id, "character_id" => character_id, "item_id" => *item_id, "looted_ts" => event.timestamp, "amount" => *amount),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /* How do are attempts and therefore rankings parsed?
     * Following rules apply:
     * 1. Attempt starts if any NPC that can start an encounter is entering combat
     * 2. Attempt ends if
     *    1a. All NPCs that are are required to die and participate, died.
     *    1b. And all NPCs that are not required to die leave combat (e.g. by dieing)
     *    2. Or if pivot NPC dies
     *    3. Or if pivot NPC goes below a certain threshold
     */
    fn extract_attempts_and_collect_ranking(&mut self, db_main: &mut (impl Execute + Select), data: &Data, now: u64) {
        static KILL_MIN_INFIGHT_UNITS: usize = 5;
        for (instance_id, committed_events) in self.committed_events.iter() {
            if let Some(UnitInstance { instance_meta_id, .. }) = self.active_instances.get(&instance_id) {
                let active_attempts = self.active_attempts.entry(*instance_id).or_insert_with(|| HashMap::with_capacity(1));
                for event in committed_events.iter() {
                    if event.timestamp + 2000 > now {
                        break;
                    }

                    match &event.subject {
                        Unit::Creature(Creature { creature_id, entry, owner: _ }) => {
                            if let Some(encounter_npc) = data.get_encounter_npc(*entry) {
                                match &event.event {
                                    EventType::CombatState { in_combat } => {
                                        if *in_combat && (active_attempts.contains_key(&encounter_npc.encounter_id) || encounter_npc.can_start_encounter) {
                                            let attempt = active_attempts.entry(encounter_npc.encounter_id).or_insert_with(|| Attempt::new(encounter_npc.encounter_id, event.timestamp));
                                            if encounter_npc.requires_death {
                                                attempt.creatures_required_to_die.insert(*creature_id);
                                            }
                                            attempt.creatures_in_combat.insert(*creature_id);
                                            if encounter_npc.is_pivot {
                                                attempt.pivot_creature = Some(*creature_id);
                                            }
                                        } else if !*in_combat {
                                            let mut is_committable = false;
                                            if let Some(attempt) = active_attempts.get_mut(&encounter_npc.encounter_id) {
                                                attempt.creatures_in_combat.remove(creature_id);
                                                is_committable = ((attempt.creatures_in_combat.is_empty() && attempt.infight_player.len() <= KILL_MIN_INFIGHT_UNITS && attempt.infight_vehicle.len() <= KILL_MIN_INFIGHT_UNITS)
                                                    || attempt.pivot_creature.contains(creature_id))
                                                    && !(encounter_npc.requires_death
                                                        && !attempt.creatures_required_to_die.is_empty()
                                                        && attempt.creatures_required_to_die.contains(creature_id)
                                                        && look_ahead_death(committed_events, event, *creature_id));
                                            }

                                            if is_committable {
                                                if let Some(mut attempt) = active_attempts.remove(&encounter_npc.encounter_id) {
                                                    attempt.end_ts = event.timestamp;
                                                    commit_attempt(db_main, *instance_meta_id, attempt);
                                                }
                                            }
                                        }
                                    },
                                    EventType::Death { murder: _ } => {
                                        let mut is_committable = false;
                                        if let Some(attempt) = active_attempts.get_mut(&encounter_npc.encounter_id) {
                                            attempt.creatures_required_to_die.remove(creature_id);
                                            if attempt.pivot_creature.contains(creature_id) {
                                                attempt.creatures_required_to_die.clear();
                                            }
                                            is_committable = attempt.creatures_required_to_die.is_empty() && attempt.infight_player.len() <= KILL_MIN_INFIGHT_UNITS && attempt.infight_vehicle.len() <= KILL_MIN_INFIGHT_UNITS;
                                        }

                                        if is_committable {
                                            if let Some(mut attempt) = active_attempts.remove(&encounter_npc.encounter_id) {
                                                attempt.end_ts = event.timestamp;
                                                commit_attempt(db_main, *instance_meta_id, attempt);
                                            }
                                        }
                                    },
                                    EventType::Power(Power { power_type, max_power, current_power }) => {
                                        if *power_type == PowerType::Health && encounter_npc.is_pivot {
                                            let mut is_committable = false;
                                            if let Some(attempt) = active_attempts.get_mut(&encounter_npc.encounter_id) {
                                                if let Some(treshold) = encounter_npc.health_treshold {
                                                    if (100 * *max_power).div(current_power) <= treshold as u32 {
                                                        attempt.creatures_required_to_die.clear();
                                                        is_committable = attempt.creatures_required_to_die.is_empty();
                                                    }
                                                }
                                            }
                                            if is_committable {
                                                if let Some(mut attempt) = active_attempts.remove(&encounter_npc.encounter_id) {
                                                    attempt.end_ts = event.timestamp;
                                                    commit_attempt(db_main, *instance_meta_id, attempt);
                                                }
                                            }
                                        }
                                    },
                                    EventType::AuraApplication(aura_app) => {
                                        if encounter_npc.health_treshold.is_none() {
                                            continue;
                                        }

                                        let mut is_committable = false;
                                        if let Some(attempt) = active_attempts.get_mut(&encounter_npc.encounter_id) {
                                            loop {
                                                {
                                                    let first_elem = attempt.pivot_instant_debuff_removes.front();
                                                    if first_elem.is_none() || event.timestamp - first_elem.unwrap() < 100 {
                                                        break;
                                                    }
                                                }
                                                attempt.pivot_instant_debuff_removes.pop_front();
                                            }

                                            if aura_app.stack_amount == 0 && attempt.pivot_creature.contains(creature_id) {
                                                attempt.pivot_instant_debuff_removes.push_back(event.timestamp);

                                                // Assume evade kill, e.g. Algalon, Freya, Hodir, Thorim
                                                is_committable = attempt.pivot_instant_debuff_removes.len() >= 15;
                                            }
                                        }

                                        if is_committable {
                                            if let Some(mut attempt) = active_attempts.remove(&encounter_npc.encounter_id) {
                                                attempt.end_ts = event.timestamp;
                                                attempt.creatures_required_to_die.clear(); // We assume death if it evades!
                                                commit_attempt(db_main, *instance_meta_id, attempt);
                                            }
                                        }
                                    },
                                    _ => {},
                                };
                            } else if let EventType::CombatState { in_combat } = &event.event {
                                for (_encounter_id, attempt) in active_attempts.iter_mut() {
                                    // Wyrmrest Skytalon
                                    if *entry == 32535 {
                                        if *in_combat {
                                            attempt.infight_vehicle.insert(*creature_id);
                                        } else {
                                            attempt.infight_vehicle.remove(&creature_id);
                                        }
                                    }
                                }
                            }
                        },
                        Unit::Player(player) => {
                            if let EventType::CombatState { in_combat } = &event.event {
                                if *in_combat {
                                    for (_encounter_id, attempt) in active_attempts.iter_mut() {
                                        attempt.infight_player.insert(player.character_id);
                                    }
                                } else {
                                    for (_encounter_id, attempt) in active_attempts.iter_mut() {
                                        attempt.infight_player.remove(&player.character_id);
                                    }
                                    // If enough player are OOC and Kill requirements are fulfilled
                                    for (encounter_id, attempt) in active_attempts.clone() {
                                        if attempt.infight_player.len() <= KILL_MIN_INFIGHT_UNITS && attempt.infight_vehicle.len() <= KILL_MIN_INFIGHT_UNITS {
                                            // Commit As Kill
                                            if attempt.creatures_required_to_die.is_empty() {
                                                if let Some(mut attempt) = active_attempts.remove(&encounter_id) {
                                                    attempt.end_ts = event.timestamp;
                                                    commit_attempt(db_main, *instance_meta_id, attempt);
                                                }
                                            }
                                            // Commit As Attempt
                                            else if attempt.creatures_in_combat.is_empty() {
                                                if let Some(mut attempt) = active_attempts.remove(&encounter_id) {
                                                    attempt.end_ts = event.timestamp;
                                                    commit_attempt(db_main, *instance_meta_id, attempt);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }

                    process_ranking(&event.subject, &event, data, active_attempts);
                }
            }
        }
    }

    fn save_current_event_id_and_end_ts(&mut self, db_main: &mut impl Execute) {
        for (instance_id, current_event_id) in self.committed_events_count.iter() {
            if let Some(UnitInstance { instance_meta_id, .. }) = self.active_instances.get(&instance_id) {
                if let Some(committed_events) = self.committed_events.get(&instance_id) {
                    if let Some(last_entry) = committed_events.back() {
                        db_main.execute_wparams(
                            "UPDATE instance_meta SET last_event_id=:current_event_id, end_ts=:end_ts WHERE id=:instance_meta_id",
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
                if let Some(extraction_index) = committable_events.iter().rposition(|event| event.timestamp + 2000 < now) {
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
                            let _ = file.write(event.deserialize().as_bytes());
                            let _ = file.write(&[10]);
                        }
                    }
                }
            }
        }
    }
}

fn process_ranking(unit: &Unit, event: &Event, data: &Data, active_attempts: &mut HashMap<u32, Attempt>) {
    if let Unit::Player(Player { character_id, .. }) = unit.get_owner_or_self() {
        match &event.event {
            EventType::SpellDamage { damage, .. } | EventType::MeleeDamage(damage) => {
                if let Unit::Creature(Creature { entry, .. }) = damage.victim {
                    if let Some(encounter_npc) = data.get_encounter_npc(entry) {
                        if let Some(attempt) = active_attempts.get_mut(&encounter_npc.encounter_id) {
                            if let Some(player_damage) = attempt.ranking_damage.get_mut(&character_id) {
                                *player_damage += get_spell_components_total(&damage.components);
                            } else {
                                attempt.ranking_damage.insert(character_id, get_spell_components_total(&damage.components));
                            }
                        }
                    }
                }
            },
            EventType::Heal { heal, .. } => {
                // TODO: We can't really tell, who this healer is in combat with
                // For now attribute heal to every attempt, though its just one in 99.9% of the cases anyway.
                // And in some cases its not even wrong, e.g. BWL where the dragons are cleaved
                for (_, attempt) in active_attempts.iter_mut() {
                    if let Some(player_heal) = attempt.ranking_heal.get_mut(&character_id) {
                        *player_heal += heal.effective;
                    } else {
                        attempt.ranking_heal.insert(character_id, heal.effective);
                    }
                }
            },
            EventType::Threat { threat, .. } => {
                if let Unit::Creature(Creature { entry, .. }) = threat.threatened {
                    if let Some(encounter_npc) = data.get_encounter_npc(entry) {
                        if let Some(attempt) = active_attempts.get_mut(&encounter_npc.encounter_id) {
                            if let Some(player_threat) = attempt.ranking_threat.get_mut(&character_id) {
                                *player_threat += threat.amount;
                            } else {
                                attempt.ranking_threat.insert(character_id, threat.amount);
                            }
                        }
                    }
                }
            },
            _ => {},
        }
    }
}

fn commit_attempt(db_main: &mut (impl Execute + Select), instance_meta_id: u32, mut attempt: Attempt) {
    // Likely a false positive
    if attempt.end_ts - attempt.start_ts <= 1000 {
        return;
    }

    let is_kill = attempt.creatures_required_to_die.is_empty();
    let params = params!("instance_meta_id" => instance_meta_id, "encounter_id" => attempt.encounter_id,
        "start_ts" => attempt.start_ts, "end_ts" => attempt.end_ts, "is_kill" => is_kill);
    db_main.execute_wparams(
        "INSERT INTO `instance_attempt` (`instance_meta_id`, `encounter_id`, `start_ts`, `end_ts`, `is_kill`) VALUES (:instance_meta_id, :encounter_id, :start_ts, :end_ts, :is_kill)",
        params.clone(),
    );

    if !is_kill {
        return;
    }

    if let Some(attempt_id) = db_main.select_wparams_value(
        "SELECT id FROM `instance_attempt` WHERE instance_meta_id=:instance_meta_id AND encounter_id=:encounter_id AND start_ts=:start_ts AND end_ts=:end_ts AND is_kill=:is_kill",
        |mut row| row.take::<u32, usize>(0),
        params,
    ) {
        let ranking_damage = std::mem::replace(&mut attempt.ranking_damage, HashMap::new());
        db_main.execute_batch_wparams(
            "INSERT INTO `instance_ranking_damage` (`character_id`, `attempt_id`, `damage`) VALUES (:character_id, :attempt_id, :damage)",
            ranking_damage.into_iter().collect(),
            move |(character_id, damage)| {
                params! {
                    "character_id" => character_id,
                    "attempt_id" => attempt_id,
                    "damage" => damage
                }
            },
        );

        let ranking_heal = std::mem::replace(&mut attempt.ranking_heal, HashMap::new());
        db_main.execute_batch_wparams(
            "INSERT INTO `instance_ranking_heal` (`character_id`, `attempt_id`, `heal`) VALUES (:character_id, :attempt_id, :heal)",
            ranking_heal.into_iter().collect(),
            move |(character_id, heal)| {
                params! {
                    "character_id" => character_id,
                    "attempt_id" => attempt_id,
                    "heal" => heal
                }
            },
        );

        let ranking_threat = std::mem::replace(&mut attempt.ranking_threat, HashMap::new());
        db_main.execute_batch_wparams(
            "INSERT INTO `instance_ranking_threat` (`character_id`, `attempt_id`, `threat`) VALUES (:character_id, :attempt_id, :threat)",
            ranking_threat.into_iter().collect(),
            move |(character_id, threat)| {
                params! {
                    "character_id" => character_id,
                    "attempt_id" => attempt_id,
                    "threat" => threat
                }
            },
        );
    }
}

fn look_ahead_death(committed_events: &VecDeque<Event>, event: &Event, creature_id: u64) -> bool {
    for la_event in committed_events.iter() {
        if la_event.id < event.id {
            continue;
        } else if la_event.timestamp - 1000 > event.timestamp {
            break;
        }
        if let EventType::Death { .. } = la_event.event {
            if let Unit::Creature(Creature { creature_id: la_creature_id, .. }) = la_event.subject {
                if la_creature_id == creature_id {
                    return true;
                }
            }
        }
    }
    false
}
