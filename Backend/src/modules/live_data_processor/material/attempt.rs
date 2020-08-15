use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone)]
pub struct Attempt {
    pub encounter_id: u32,
    pub start_ts: u64,
    pub end_ts: u64,
    pub pivot_creature: Option<u64>, // (CreatureId, died)
    pub creatures_required_to_die: BTreeSet<u64>,
    pub creatures_in_combat: BTreeSet<u64>,
    pub ranking_damage: HashMap<u32, u32>,
    pub ranking_heal: HashMap<u32, u32>,
    pub ranking_threat: HashMap<u32, i32>,
}

impl Attempt {
    pub fn new(encounter_id: u32, start_ts: u64) -> Self {
        Attempt {
            encounter_id,
            start_ts,
            end_ts: 0,
            pivot_creature: None,
            creatures_required_to_die: BTreeSet::new(),
            creatures_in_combat: BTreeSet::new(),
            ranking_damage: HashMap::new(),
            ranking_heal: HashMap::new(),
            ranking_threat: HashMap::new(),
        }
    }
}
