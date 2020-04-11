use crate::modules::live_data_processor::dto::{UnAura, Message, MessageType};
use crate::modules::live_data_processor::domain_value::{Unit, Event, EventType};
use crate::modules::live_data_processor::tools::MapUnit;
use std::collections::{BTreeSet, HashMap};
use crate::modules::armory::Armory;

/// There is a SpellCast event that steals an AuraApplication event
/// Note: un_aura_spell_id is currently constant 0
// TODO: We need to also consider the case that we escaped the loop because it ended and not because the time threshold was reached
pub fn try_parse_spell_steal(non_committed_messages: &mut Vec<Message>, committed_events: &Vec<Event>, timestamp: u64, spell_steal: &UnAura, subject: &Unit, armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Option<(u32, u32)> {
  let target = spell_steal.target.to_unit(armory, server_id, summons);
  if target.is_err() {
    println!("Could not parse this spell steal: {:?}", spell_steal);
    non_committed_messages.pop().unwrap();
    return None;
  }
  let target = target.unwrap();

  let aura_caster = spell_steal.aura_caster.to_unit(armory, server_id, summons);
  if aura_caster.is_err() {
    println!("Could not parse this spell steal: {:?}", spell_steal);
    non_committed_messages.pop().unwrap();
    return None;
  }
  let aura_caster = aura_caster.unwrap();


  let mut spell_cast_event_id = None;
  let mut aura_application_event_id = None;

  // Case 1: There was a matching pair of events in the past
  for i in (0..committed_events.len()).rev() {
    let event: &Event = committed_events.get(i).unwrap();
    if (timestamp as i64 - event.timestamp as i64).abs() > 10 {
      break;
    }
    match &event.event {
      EventType::SpellCast(spell_cast) => {
        if let Some(spell_id) = &spell_cast.spell_id {
          if let Some(victim) = &spell_cast.victim {
            if victim == &target
              && &event.subject == subject
              && is_spell_steal(*spell_id) {
              spell_cast_event_id = Some(event.id);
            }
          }
        }
      },
      EventType::AuraApplication(aura_application) => {
        if event.subject == target
          && aura_application.caster == aura_caster
          && aura_application.spell_id == spell_steal.target_spell_id
          && aura_application.stack_amount > 0 {
          aura_application_event_id = Some(event.id);
        }
      }
      _ => continue
    };

    if spell_cast_event_id.is_some() && aura_application_event_id.is_some() {
      return Some((spell_cast_event_id.unwrap(), aura_application_event_id.unwrap()));
    }
  }

  // Case 2: The matching event is in the future and we need to reorder
  let mut spell_cast_msg_index = None;
  let mut aura_application_msg_index = None;
  for (msg_index, msg) in non_committed_messages.iter().enumerate() {
    if (msg.timestamp as i64 - timestamp as i64).abs() > 10 {
      break;
    }
    match &msg.message_type {
      MessageType::SpellCast(spell_cast) => {
        if spell_cast_msg_index.is_none()
          && spell_cast_event_id.is_none() {
          if let Some(target) = &spell_cast.target {
            if target == &spell_steal.target
              && spell_cast.caster == spell_steal.un_aura_caster
              && is_spell_steal(spell_cast.spell_id) {
              spell_cast_msg_index = Some(msg_index);
            }
          }
        }
      },
      MessageType::AuraApplication(aura_application) => {
        if aura_application_msg_index.is_none()
          && aura_application_event_id.is_none()
          && aura_application.applied
          && aura_application.caster == spell_steal.aura_caster
          && aura_application.target == spell_steal.target
          && aura_application.spell_id == spell_steal.target_spell_id {
          aura_application_msg_index = Some(msg_index);
        }
      },
      _ => continue
    };
  }

  // There are in total 4 cases, maybe there is a more elegant way to do this
  if let Some(spell_cast_index) = spell_cast_msg_index {
    if let Some(aura_application_index) = aura_application_msg_index {
      if spell_cast_index > aura_application_index {
        let removed_spell_cast = non_committed_messages.remove(spell_cast_index);
        let removed_aura_application = non_committed_messages.remove(aura_application_index);
        non_committed_messages.insert(0, removed_spell_cast);
        non_committed_messages.insert(0, removed_aura_application);
      } else {
        let removed_aura_application = non_committed_messages.remove(aura_application_index);
        let removed_spell_cast = non_committed_messages.remove(spell_cast_index);
        non_committed_messages.insert(0, removed_spell_cast);
        non_committed_messages.insert(0, removed_aura_application);
      }
    } else {
      let removed_msg = non_committed_messages.remove(spell_cast_index);
      non_committed_messages.insert(0, removed_msg);
    }
  } else {
    if let Some(aura_application_index) = aura_application_msg_index {
      let removed_msg = non_committed_messages.remove(aura_application_index);
      non_committed_messages.insert(0, removed_msg);
    } else {
      // Well what do we do now :O
      println!("Could not parse this spell steal: {:?}", spell_steal);
      non_committed_messages.pop().unwrap();
    }
  }
  None
}

fn is_spell_steal(spell_id: u32) -> bool {
  lazy_static! {
    static ref SPELL_STEAL_SPELLS: BTreeSet<u32> = [].iter().cloned().collect();
  }
  SPELL_STEAL_SPELLS.contains(&spell_id)
}