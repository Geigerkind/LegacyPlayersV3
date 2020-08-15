use crate::modules::data::{domain_value::Encounter, Data};

pub trait RetrieveEncounter {
    fn get_encounter(&self, id: u32) -> Option<Encounter>;
    fn get_all_encounters(&self) -> Vec<Encounter>;
}

impl RetrieveEncounter for Data {
    fn get_encounter(&self, id: u32) -> Option<Encounter> {
        self.encounters.get(&id).cloned()
    }

    fn get_all_encounters(&self) -> Vec<Encounter> {
        self.encounters.iter().map(|(_, encounter)| encounter.clone()).collect()
    }
}
