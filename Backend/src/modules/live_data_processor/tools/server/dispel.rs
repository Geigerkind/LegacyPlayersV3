use crate::modules::live_data_processor::dto::{Message, UnAura, MessageType};
use crate::modules::live_data_processor::domain_value::{Event, Unit, EventType};
use crate::modules::armory::Armory;
use std::collections::HashMap;
use crate::modules::live_data_processor::tools::MapUnit;

pub fn try_parse_dispel(non_committed_messages: &mut Vec<Message>, committed_events: &[Event], timestamp: u64, dispel: &UnAura, subject: &Unit, armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Option<(u32, Vec<u32>)> {
  let target = dispel.target.to_unit(armory, server_id, summons);
  if target.is_err() {
    println!("Could not parse this dispel: {:?}", dispel);
    non_committed_messages.pop().unwrap();
    return None;
  }
  let target = target.unwrap();

  let aura_caster = dispel.aura_caster.to_unit(armory, server_id, summons);
  if aura_caster.is_err() {
    println!("Could not parse this dispel: {:?}", dispel);
    non_committed_messages.pop().unwrap();
    return None;
  }
  let aura_caster = aura_caster.unwrap();

  let mut spell_cast_event_id = None;
  let mut aura_application_event_ids = Vec::new();

  // Collect what is committed
  for i in (0..committed_events.len()).rev() {
    let event: &Event = committed_events.get(i).unwrap();
    if (timestamp as i64 - event.timestamp as i64).abs() > 10 {
      break;
    }
    match &event.event {
      // I wonder if all hot dispel casts have the same spell id as the hot in the end
      EventType::SpellCast(spell_cast) => {
        if spell_cast_event_id.is_some() {
          continue;
        }

        if let Some(spell_id) = &spell_cast.spell_id {
          if let Some(victim) = &spell_cast.victim {
            if victim == &target
              && &event.subject == subject
              && dispel.un_aura_spell_id == *spell_id {
              spell_cast_event_id = Some(event.id);
            }
          }
        }
      },
      EventType::AuraApplication(aura_application) => {
        if aura_application_event_ids.len() < dispel.un_aura_amount as usize
          && event.subject == target
          && aura_application.caster == aura_caster
          && aura_application.spell_id == dispel.target_spell_id {
          aura_application_event_ids.push(event.id);
        }
      }
      _ => continue
    };

    if let Some(spell_cast_event_id) = spell_cast_event_id {
      if aura_application_event_ids.len() == dispel.un_aura_amount as usize {
        return Some((spell_cast_event_id, aura_application_event_ids));
      }
    }
  }

  // Find out what is not committed
  let mut spell_cast_found = false;
  let mut num_dispels_found = aura_application_event_ids.len();
  let mut msg_indexes = Vec::new();
  for (msg_index, msg) in non_committed_messages.iter().enumerate() {
    if (msg.timestamp as i64 - timestamp as i64).abs() > 10 {
      break;
    }
    match &msg.message_type {
      MessageType::SpellCast(spell_cast) => {
        if !spell_cast_found
          && spell_cast_event_id.is_none() {
          if let Some(target) = &spell_cast.target {
            if target == &dispel.target
              && spell_cast.caster == dispel.un_aura_caster
              && spell_cast.spell_id == dispel.un_aura_spell_id {
              spell_cast_found = true;
              msg_indexes.push(msg_index);
            }
          }
        }
      },
      MessageType::AuraApplication(aura_application) => {
        if num_dispels_found == dispel.un_aura_amount as usize
          && !aura_application.applied
          && aura_application.caster == dispel.aura_caster
          && aura_application.target == dispel.target
          && aura_application.spell_id == dispel.target_spell_id {
          num_dispels_found += 1;
          msg_indexes.push(msg_index);
        }
      },
      _ => continue
    };
  }

  // Prepend uncommitted events
  for (counter, msg_index) in msg_indexes.iter().rev().enumerate() {
    let msg = non_committed_messages.remove(*msg_index + counter);
    non_committed_messages.insert(0, msg);
  }

  None
}