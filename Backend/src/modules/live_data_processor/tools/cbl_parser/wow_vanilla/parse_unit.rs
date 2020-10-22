use crate::modules::data::Data;
use crate::modules::live_data_processor::dto::Unit;
use crate::modules::live_data_processor::tools::cbl_parser::wow_vanilla::hashed_unit_id::{get_hashed_player_unit_id, get_npc_unit_id};

pub fn parse_unit(data: &Data, unit_name: &str) -> Unit {
    if let Some(unit_id) = get_npc_unit_id(data, &unit_name.to_string()) {
        Unit { is_player: false, unit_id }
    } else {
        Unit {
            is_player: true,
            unit_id: get_hashed_player_unit_id(unit_name),
        }
    }
}
