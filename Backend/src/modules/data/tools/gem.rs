use crate::modules::data::{domain_value::Gem, Data};

pub trait RetrieveGem {
    fn get_gem(&self, expansion_id: u8, item_id: u32) -> Option<Gem>;
}

impl RetrieveGem for Data {
    fn get_gem(&self, expansion_id: u8, item_id: u32) -> Option<Gem> {
        if expansion_id <= 1 {
            return None;
        }

        // Gems are introduced with TBC
        self.gems.get(expansion_id as usize - 2).and_then(|map| map.get(&item_id).cloned())
    }
}
