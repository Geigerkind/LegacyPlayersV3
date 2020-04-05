use crate::modules::live_data_processor::dto::{MessageType, Message};
use crate::modules::live_data_processor::domain_value::{HitType, Mitigation, Damage, School, SpellCast, Heal};
use crate::modules::live_data_processor::tools::MapUnit;

/// ## What has to be done here?
/// * We have to collect a spell cast
///   * This can have optional heal, damage done and threat
/// * If we have a spell cast but none other and the timeframe runs out,
///   then we will discard potential other and commit the spell cast
/// * NOTE: Melee damage does not have casts, but we will treat it as
///   a spell internally anyway
/// * TODO: Threat needs to have someone that is THREATENED
pub fn try_parse_spell_cast(non_committed_messages: &mut Vec<Message>, first_message: &Message) -> Option<SpellCast> {
  let mut spell_cast_message = None;
  let mut spell_damage_done_message = None;
  let mut melee_damage_done_message = None;
  let mut threat_message = None;
  let mut heal_message = None;
  let mut reached_timeout = false;
  for msg in non_committed_messages.iter() {
    if msg.timestamp - first_message.timestamp > 50 {
      reached_timeout = true;
      break;
    } else if !spell_matches(&first_message, &msg) {
      continue;
    }

    match msg.message_type {
      MessageType::SpellCast(_) => spell_cast_message = Some(msg.clone()),
      MessageType::SpellDamage(_) => spell_damage_done_message = Some(msg.clone()),
      MessageType::MeleeDamage(_) => melee_damage_done_message = Some(msg.clone()),
      MessageType::Threat(_) => threat_message = Some(msg.clone()),
      MessageType::Heal(_) => heal_message = Some(msg.clone()),
      _ => continue
    }
  }

  if let Some(melee_damage_done_message) = melee_damage_done_message {
    if let MessageType::MeleeDamage(melee_damage) = &melee_damage_done_message.message_type {
      if reached_timeout || threat_message.is_some() {
        non_committed_messages.remove_item(&melee_damage_done_message).expect("Should be deleted!");
        let mut mitigation = Vec::new();
        if melee_damage.blocked > 0 { mitigation.push(Mitigation::Block(melee_damage.blocked)); }
        if melee_damage.absorbed > 0 { mitigation.push(Mitigation::Absorb(melee_damage.absorbed)); }
        if melee_damage.resisted_or_glanced > 0 { mitigation.push(Mitigation::Glance(melee_damage.resisted_or_glanced)); }

        return Some(SpellCast {
          victim: melee_damage.victim.to_unit().ok(),
          hit_type: HitType::from_u8(melee_damage.hit_type.expect("Must exist for melee attacks!")),
          spell_id: None,
          damage: Some(Damage {
            school: School::from_u8(melee_damage.school),
            damage: melee_damage.damage,
            mitigation
          }),
          heal: None,
          threat: threat_message.and_then(|message| {
            if let MessageType::Threat(threat) = &message.message_type {
              non_committed_messages.remove_item(&message).expect("Should be deleted!");
              Some(threat.amount)
            } else { None }
          })
        });
      }
      return None;
    }
  } else if let Some(spell_cast_message) = spell_cast_message {
    if let MessageType::SpellCast(spell_cast) = &spell_cast_message.message_type {
      if reached_timeout || (threat_message.is_some() && (spell_damage_done_message.is_some() || heal_message.is_some())) {
        non_committed_messages.remove_item(&spell_cast_message).expect("Should be deleted!");
        return Some(SpellCast {
          victim: spell_cast.target.map(|target| target.to_unit().expect("Must be an Unit")), // TODO: Handle this in the future
          hit_type: HitType::from_u8(spell_cast.hit_type), // TODO: CHeck if this matches!
          spell_id: Some(spell_cast.spell_id),
          damage: spell_damage_done_message.and_then(|message| {
            // Always true
            if let MessageType::SpellDamage(spell_damage) = &message.message_type {
              non_committed_messages.remove_item(&message).expect("Should be deleted!");
              let mut mitigation = Vec::new();
              if spell_damage.blocked > 0 { mitigation.push(Mitigation::Block(spell_damage.blocked)); }
              if spell_damage.absorbed > 0 { mitigation.push(Mitigation::Absorb(spell_damage.absorbed)); }
              if spell_damage.resisted_or_glanced > 0 { mitigation.push(Mitigation::Resist(spell_damage.resisted_or_glanced)); }
              return Some(Damage {
                school: School::from_u8(spell_damage.school),
                damage: spell_damage.damage,
                mitigation
              });
            }
            None
          }),
          heal: heal_message.and_then(|message| {
            // Always true
            if let MessageType::Heal(heal) = &message.message_type {
              non_committed_messages.remove_item(&message).expect("Should be deleted!");
              return Some(Heal {
                total: heal.total_heal,
                effective: heal.effective_heal,
                mitigation: if heal.absorb > 0 { vec![Mitigation::Absorb(heal.absorb)] } else { Vec::new() }
              })
            }
            None
          }),
          threat: threat_message.and_then(|message| {
            if let MessageType::Threat(threat) = &message.message_type {
              non_committed_messages.remove_item(&message).expect("Should be deleted!");
              Some(threat.amount)
            } else { None }
          })
        });
      }
      return None;
    }
  }

  // Timeout, remove these messages then!
  if reached_timeout {
    if let Some(message) = threat_message { non_committed_messages.remove_item(&message).expect("Should be deleted!"); }
    if let Some(message) = spell_damage_done_message { non_committed_messages.remove_item(&message).expect("Should be deleted!"); }
    if let Some(message) = heal_message { non_committed_messages.remove_item(&message).expect("Should be deleted!"); }
  }
  None
}

fn extract_spell_attacker(message_type: &MessageType) -> Option<u64> {
  match message_type {
    MessageType::MeleeDamage(item) => Some(item.attacker),
    MessageType::SpellDamage(item) => Some(item.attacker),
    MessageType::Heal(item) => Some(item.caster),
    MessageType::SpellCast(item) => Some(item.caster),
    MessageType::Threat(item) => Some(item.threater),
    _ => None
  }
}
fn extract_spell_target(message_type: &MessageType) -> Option<u64> {
  match message_type {
    MessageType::MeleeDamage(item) => Some(item.victim),
    MessageType::SpellDamage(item) => Some(item.victim),
    MessageType::Heal(item) => Some(item.target),
    MessageType::SpellCast(item) => item.target,
    MessageType::Threat(item) => Some(item.threatened),
    _ => None
  }
}
fn extract_spell_id(message_type: &MessageType) -> Option<u32> {
  match message_type {
    MessageType::MeleeDamage(item) => item.spell_id,
    MessageType::SpellDamage(item) => item.spell_id,
    MessageType::Heal(item) => Some(item.spell_id),
    MessageType::SpellCast(item) => Some(item.spell_id),
    MessageType::Threat(item) => item.spell_id,
    _ => None
  }
}
fn spell_matches(msg1: &Message, msg2: &Message) -> bool {
  extract_spell_attacker(&msg2.message_type) == extract_spell_attacker(&msg1.message_type)
    && extract_spell_target(&msg2.message_type) == extract_spell_target(&msg1.message_type)
    && extract_spell_id(&msg2.message_type) == extract_spell_id(&msg1.message_type)
}