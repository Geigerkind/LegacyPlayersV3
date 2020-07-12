use crate::modules::armory::Armory;
use crate::modules::live_data_processor::domain_value::{Event, EventParseFailureAction, EventType};
use crate::modules::live_data_processor::dto::UnAura;
use crate::modules::live_data_processor::tools::MapUnit;
use std::collections::{BTreeSet, HashMap};
use crate::util::database::{Execute, Select};

/// There is a SpellCast event that steals an AuraApplication event
/// Note: un_aura_spell_id is currently constant 0
pub fn try_parse_spell_steal(db_main: &mut (impl Select + Execute), spell_steal: &UnAura, committed_events: &[Event], timestamp: u64, next_timestamp: u64, armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Result<(u32, u32), EventParseFailureAction> {
    let aura_caster = spell_steal.aura_caster.to_unit(db_main, armory, server_id, summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
    let target = spell_steal.target.to_unit(db_main, armory, server_id, summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;

    let mut spell_cast_event_id = None;
    let mut aura_application_event_id = None;

    // Case 1: There was a matching pair of events in the past
    for i in (0..committed_events.len()).rev() {
        let event: &Event = committed_events.get(i).unwrap();
        if (timestamp as i64 - event.timestamp as i64).abs() > 10 {
            break;
        }

        if let Some(spell_cast_event_id) = spell_cast_event_id {
            if let Some(aura_application_event_id) = aura_application_event_id {
                return Ok((spell_cast_event_id, aura_application_event_id));
            }
        }

        match &event.event {
            EventType::SpellCast(spell_cast) => {
                if let Some(spell_id) = &spell_cast.spell_id {
                    if let Some(victim) = &spell_cast.victim {
                        if victim == &target && event.subject == aura_caster && is_spell_steal(*spell_id) {
                            spell_cast_event_id = Some(event.id);
                        }
                    }
                }
            },
            EventType::AuraApplication(aura_application) => {
                if event.subject == target && aura_application.spell_id == spell_steal.target_spell_id && aura_application.stack_amount > 0 {
                    aura_application_event_id = Some(event.id);
                }
            },
            _ => continue,
        };
    }

    if next_timestamp as i64 - timestamp as i64 > 10 {
        return Err(EventParseFailureAction::DiscardFirst);
    }

    Err(EventParseFailureAction::Wait)
}

fn is_spell_steal(spell_id: u32) -> bool {
    lazy_static! {
        static ref SPELL_STEAL_SPELLS: BTreeSet<u32> = [30449].iter().cloned().collect();
    }
    SPELL_STEAL_SPELLS.contains(&spell_id)
}
