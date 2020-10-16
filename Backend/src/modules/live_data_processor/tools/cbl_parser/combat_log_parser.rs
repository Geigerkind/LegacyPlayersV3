use crate::modules::live_data_processor::dto::MessageType;
use crate::modules::armory::dto::CharacterDto;
use crate::modules::live_data_processor::material::{Participant, ActiveMapVec};
use crate::modules::data::Data;

pub trait CombatLogParser {
    fn parse_cbl_line(&mut self, data: &Data, event_ts: u64, content: &str) -> Option<Vec<MessageType>>;
    // Server that need to be created have Id=0!
    fn get_involved_server(&self) -> Option<Vec<(u32, String, String)>>;
    fn get_involved_character_builds(&self) -> Vec<(Option<u32>, CharacterDto)>;
    fn get_participants(&self) -> Vec<Participant>;
    fn get_active_maps(&self) -> ActiveMapVec;
    fn get_npc_appearance_offset(&self, entry: u32) -> Option<i64>;
    fn get_npc_timeout(&self, entry: u32) -> Option<u64>;
    // Source entry causes InCombat=True event for all resulting npc entries with an offset
    fn get_death_implied_npc_combat_state_and_offset(&self, entry: u32) -> Option<Vec<(u32, i64)>>;
    // In combat of source npc implies in combat of resulting npcs
    fn get_in_combat_implied_npc_combat(&self, entry: u32) -> Option<Vec<u32>>;
    fn get_expansion_id(&self) -> u8;
    fn get_server_id(&self) -> Option<u32>;
}