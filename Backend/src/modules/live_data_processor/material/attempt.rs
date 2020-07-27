use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Attempt {
    pub is_kill: bool,
    pub creature_id: u64,
    pub npc_id: u32,
    pub start_ts: u64,
    pub end_ts: u64,
    pub ranking_damage: HashMap<u32, u32>,
    pub ranking_heal: HashMap<u32, u32>,
    pub ranking_threat: HashMap<u32, i32>,
}

impl Attempt {
    pub fn new(creature_id: u64, npc_id: u32, start_ts: u64) -> Self {
        Attempt {
            is_kill: false,
            creature_id,
            npc_id,
            start_ts,
            end_ts: 0,
            ranking_damage: HashMap::new(),
            ranking_heal: HashMap::new(),
            ranking_threat: HashMap::new(),
        }
    }
}
