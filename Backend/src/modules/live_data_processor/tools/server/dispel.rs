use crate::modules::live_data_processor::dto::{Message, UnAura};
use crate::modules::live_data_processor::domain_value::{Event, Unit};
use crate::modules::armory::Armory;
use std::collections::HashMap;

pub fn try_parse_dispel(non_committed_messages: &mut Vec<Message>, committed_events: &Vec<Event>, timestamp: u64, spell_steal: &UnAura, subject: &Unit, armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Option<(u32, Vec<u32>)> {
  unimplemented!()
}