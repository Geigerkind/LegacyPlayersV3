use crate::modules::armory::tools::GetCharacter;
use crate::modules::armory::Armory;
use crate::modules::live_data_processor::domain_value;
use crate::modules::live_data_processor::dto;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::tools::GUID;
use std::collections::HashMap;

pub trait MapUnit {
    fn to_unit(&self, armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Result<domain_value::Unit, LiveDataProcessorFailure>;
}

impl MapUnit for dto::Unit {
    fn to_unit(&self, armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Result<domain_value::Unit, LiveDataProcessorFailure> {
        if self.is_player {
            // TODO: Create 'empty' character here!
            let character = armory.get_character_by_uid(server_id, self.unit_id).expect("TODO: Create 'empty' character here!");
            Ok(domain_value::Unit::Player(domain_value::Player {
                character_id: character.id,
                server_uid: character.server_uid
            }))
        } else {
            Ok(domain_value::Unit::Creature(domain_value::Creature {
                creature_id: self.unit_id,
                entry: self.unit_id.get_entry().expect("Non player should be a unit"),
                owner: summons.get(&self.unit_id).cloned(),
            }))
        }
    }
}
