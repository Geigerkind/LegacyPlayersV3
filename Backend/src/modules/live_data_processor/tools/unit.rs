use crate::modules::armory::tools::{CreateCharacter, GetCharacter};
use crate::modules::armory::Armory;
use crate::modules::live_data_processor::domain_value;
use crate::modules::live_data_processor::dto;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::tools::GUID;
use crate::util::database::{Execute, Select};
use std::collections::HashMap;

pub trait MapUnit {
    fn to_unit(&self, db_main: &mut (impl Execute + Select), armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Result<domain_value::Unit, LiveDataProcessorFailure>;
}

impl MapUnit for dto::Unit {
    fn to_unit(&self, db_main: &mut (impl Execute + Select), armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Result<domain_value::Unit, LiveDataProcessorFailure> {
        if self.is_player {
            let mut character_id = armory.get_character_by_uid(server_id, self.unit_id).map(|character| character.id);
            if character_id.is_none() {
                character_id = armory.create_character(db_main, server_id, self.unit_id).ok();
            }
            Ok(domain_value::Unit::Player(domain_value::Player {
                character_id: character_id.ok_or_else(|| LiveDataProcessorFailure::InvalidInput)?,
                server_uid: self.unit_id,
            }))
        } else {
            Ok(domain_value::Unit::Creature(domain_value::Creature {
                creature_id: self.unit_id,
                entry: self.unit_id.get_entry().ok_or_else(|| LiveDataProcessorFailure::InvalidInput)?,
                owner: summons.get(&self.unit_id).cloned(),
            }))
        }
    }
}
