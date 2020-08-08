use crate::modules::data::{domain_value::EncounterNpc, Data};

pub trait RetrieveEncounterNpc {
    fn get_encounter_npc(&self, id: u32) -> Option<EncounterNpc>;
    fn get_all_encounter_npcs(&self) -> Vec<EncounterNpc>;
}

impl RetrieveEncounterNpc for Data {
    fn get_encounter_npc(&self, id: u32) -> Option<EncounterNpc> {
        self.encounter_npcs.get(&id).cloned()
    }

    fn get_all_encounter_npcs(&self) -> Vec<EncounterNpc> {
        self.encounter_npcs.iter().map(|(_, encounter_npc)| encounter_npc.clone()).collect()
    }
}
