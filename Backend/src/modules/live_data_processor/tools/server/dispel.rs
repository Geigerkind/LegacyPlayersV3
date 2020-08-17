use crate::modules::armory::Armory;
use crate::modules::live_data_processor::domain_value::{Event, EventParseFailureAction, EventType};
use crate::modules::live_data_processor::dto::UnAura;
use crate::modules::live_data_processor::tools::MapUnit;
use crate::util::database::{Execute, Select};
use std::collections::{HashMap, VecDeque};

pub fn try_parse_dispel(
    db_main: &mut (impl Select + Execute), dispel: &UnAura, recently_committed_spell_cast_and_aura_applications: &VecDeque<Event>, armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>,
) -> Result<(u32, u32), EventParseFailureAction> {
    let un_aura_caster = dispel.un_aura_caster.to_unit(db_main, armory, server_id, summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
    let target = dispel.target.to_unit(db_main, armory, server_id, summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;

    let mut un_aura_event_id = None;
    let mut aura_application_event_id = None;

    for i in (0..recently_committed_spell_cast_and_aura_applications.len()).rev() {
        let event: &Event = recently_committed_spell_cast_and_aura_applications.get(i).unwrap();

        if let Some(un_aura_event_id) = un_aura_event_id {
            if let Some(aura_app_event_id) = aura_application_event_id {
                return Ok((un_aura_event_id, aura_app_event_id));
            }
        }

        match &event.event {
            // I wonder if all hot dispel casts have the same spell id as the hot in the end
            EventType::SpellCast(spell_cast) => {
                if un_aura_event_id.is_some() {
                    continue;
                }

                if let Some(victim) = &spell_cast.victim {
                    if victim == &target && event.subject == un_aura_caster && dispel.un_aura_spell_id == spell_cast.spell_id {
                        un_aura_event_id = Some(event.id);
                    }
                }
            },
            EventType::AuraApplication(aura_application) => {
                // TODO: How much did the aura app change?
                if event.subject == target && aura_application.spell_id == dispel.target_spell_id {
                    if aura_application_event_id.is_some() {
                        continue;
                    }

                    aura_application_event_id = Some(event.id);
                } else if aura_application.caster == un_aura_caster && event.subject == target && aura_application.spell_id == dispel.un_aura_spell_id {
                    if un_aura_event_id.is_some() {
                        continue;
                    }

                    un_aura_event_id = Some(event.id);
                }
            },
            _ => continue,
        };
    }

    if let Some(un_aura_event_id) = un_aura_event_id {
        if let Some(aura_app_event_id) = aura_application_event_id {
            return Ok((un_aura_event_id, aura_app_event_id));
        }
    }
    Err(EventParseFailureAction::PrependNext)
}
