use crate::modules::armory::Armory;
use crate::modules::live_data_processor::domain_value::{Event, EventParseFailureAction, EventType};
use crate::modules::live_data_processor::dto::UnAura;
use crate::modules::live_data_processor::tools::MapUnit;
use crate::util::database::{Execute, Select};
use std::collections::{HashMap, VecDeque};

pub fn try_parse_dispel(db_main: &mut (impl Select + Execute), dispel: &UnAura, committed_events: &VecDeque<Event>, timestamp: u64, armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Result<(u32, Vec<u32>), EventParseFailureAction> {
    let un_aura_caster = dispel.un_aura_caster.to_unit(db_main, armory, server_id, summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
    let target = dispel.target.to_unit(db_main, armory, server_id, summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;

    let mut spell_cast_event_id = None;
    let mut aura_application_event_ids = Vec::new();

    for i in (0..committed_events.len()).rev() {
        let event: &Event = committed_events.get(i).unwrap();
        if (timestamp as i64 - event.timestamp as i64).abs() > 10 {
            break;
        }

        if let Some(spell_cast_event_id) = spell_cast_event_id {
            if aura_application_event_ids.len() == dispel.un_aura_amount as usize {
                return Ok((spell_cast_event_id, aura_application_event_ids));
            }
        }

        match &event.event {
            // I wonder if all hot dispel casts have the same spell id as the hot in the end
            EventType::SpellCast(spell_cast) => {
                if spell_cast_event_id.is_some() {
                    continue;
                }

                if let Some(victim) = &spell_cast.victim {
                    if victim == &target && event.subject == un_aura_caster && dispel.un_aura_spell_id == spell_cast.spell_id {
                        spell_cast_event_id = Some(event.id);
                    }
                }
            },
            EventType::AuraApplication(aura_application) => {
                // TODO: How much did the aura app change?
                if aura_application_event_ids.len() < dispel.un_aura_amount as usize && event.subject == target && aura_application.spell_id == dispel.target_spell_id {
                    aura_application_event_ids.push(event.id);
                }
            },
            _ => continue,
        };
    }

    if let Some(spell_cast_event_id) = spell_cast_event_id {
        if !aura_application_event_ids.is_empty() {
            return Ok((spell_cast_event_id, aura_application_event_ids));
        }
    }
    Err(EventParseFailureAction::PrependNext)
}
