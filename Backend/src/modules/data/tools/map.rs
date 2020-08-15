use crate::modules::data::domain_value::Map;
use crate::modules::data::Data;

pub trait RetrieveMap {
    fn get_map(&self, id: u16) -> Option<Map>;
    fn get_all_maps(&self) -> Vec<Map>;
}

impl RetrieveMap for Data {
    fn get_map(&self, id: u16) -> Option<Map> {
        self.maps.get(&id).cloned()
    }

    fn get_all_maps(&self) -> Vec<Map> {
        self.maps.iter().map(|(_, map)| map.clone()).collect()
    }
}
