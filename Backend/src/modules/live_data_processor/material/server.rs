use crate::modules::live_data_processor::domain_value::{Event, NonCommittedEvent, UnitInstance};
use crate::modules::live_data_processor::dto::InstanceResetDto;
use crate::params;
use crate::util::database::Select;
use std::collections::{BTreeSet, HashMap};

pub struct Server {
    pub server_id: u32,

    // Meta Data
    pub summons: HashMap<u64, u64>,
    // TODO: This grows uncontrollable
    pub active_instances: HashMap<u32, UnitInstance>,
    // TODO: How to deal with changing difficulties in WOTLK?
    pub unit_instance_id: HashMap<u64, u32>,
    pub instance_resets: HashMap<u16, InstanceResetDto>,
    // instance_meta_id => [(character_id, history_id)]
    pub instance_participants: HashMap<u32, HashMap<u32, Option<u32>>>,

    // Used to handle unordered events
    pub subject_prepend_mode_set: BTreeSet<u64>, // Contains server_uid of subject

    // Events
    // Mapping player to non committed event
    pub non_committed_events: HashMap<u64, NonCommittedEvent>,
    // Instance_id => Events
    pub committed_events: HashMap<u32, Vec<Event>>,
    pub committed_events_count: HashMap<u32, u32>,
}

impl Server {
    pub fn new(server_id: u32) -> Self {
        Server {
            server_id,
            // TODO: Preserve all data when the app is restarted?
            summons: HashMap::new(),
            active_instances: HashMap::new(),
            unit_instance_id: HashMap::new(),
            instance_resets: HashMap::new(),
            instance_participants: HashMap::new(),
            non_committed_events: HashMap::new(),
            committed_events: HashMap::new(),
            committed_events_count: HashMap::new(),
            subject_prepend_mode_set: BTreeSet::new(),
        }
    }

    pub fn init(mut self, db_main: &mut impl Select) -> Self {
        // Load active instances
        db_main
            .select_wparams(
                "SELECT id, start_ts, map_id, instance_id FROM instance_meta WHERE expired=0 AND server_id=:server_id",
                |mut row| UnitInstance {
                    instance_meta_id: row.take(0).unwrap(),
                    entered: row.take(1).unwrap(),
                    map_id: row.take(2).unwrap(),
                    instance_id: row.take(3).unwrap(),
                },
                params!("server_id" => self.server_id),
            )
            .into_iter()
            .for_each(|unit_instance| {
                self.instance_participants.insert(unit_instance.instance_meta_id, HashMap::new());
                self.active_instances.insert(unit_instance.instance_id, unit_instance);
            });

        // Load active instance participants
        db_main
            .select_wparams(
                "SELECT A.id, B.character_id, B.history_id FROM instance_meta A JOIN instance_participants B ON A.id = B.instance_meta_id WHERE A.expired = 0 AND A.server_id=:server_id",
                |mut row| (row.take::<u32, usize>(0).unwrap(), row.take::<u32, usize>(1).unwrap(), row.take_opt::<u32, usize>(2).unwrap().ok()),
                params!("server_id" => self.server_id),
            )
            .into_iter()
            .for_each(|(instance_meta_id, character_id, history_id)| {
                self.instance_participants.get_mut(&instance_meta_id).unwrap().insert(character_id, history_id);
            });

        // Load instance reset data
        db_main
            .select_wparams(
                "SELECT map_id, difficulty, reset_time FROM armory_instance_resets WHERE server_id=:server_id",
                |mut row| InstanceResetDto {
                    map_id: row.take(0).unwrap(),
                    difficulty: row.take(1).unwrap(),
                    reset_time: row.take(2).unwrap(),
                },
                params!("server_id" => self.server_id),
            )
            .into_iter()
            .for_each(|result| {
                self.instance_resets.insert(result.map_id, result);
            });
        self
    }
}
