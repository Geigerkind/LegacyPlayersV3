use crate::modules::data::domain_value::ItemDamageType;
use crate::modules::data::Data;

pub trait RetrieveItemDamageType {
    fn get_item_damage_type(&self, id: u8) -> Option<ItemDamageType>;
    fn get_all_item_damage_types(&self) -> Vec<ItemDamageType>;
}

impl RetrieveItemDamageType for Data {
    fn get_item_damage_type(&self, id: u8) -> Option<ItemDamageType> {
        self.item_damage_types
            .get(&id)
            .and_then(|item_damage_type| Some(item_damage_type.clone()))
    }

    fn get_all_item_damage_types(&self) -> Vec<ItemDamageType> {
        self.item_damage_types
            .iter()
            .map(|(_, item_damage_type)| item_damage_type.clone())
            .collect()
    }
}
