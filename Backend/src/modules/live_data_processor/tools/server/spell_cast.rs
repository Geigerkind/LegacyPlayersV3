use crate::modules::armory::Armory;
use crate::modules::live_data_processor::domain_value;
use crate::modules::live_data_processor::domain_value::{Damage, EventParseFailureAction, Heal, HitType, Mitigation, School, SpellCast, Threat};
use crate::modules::live_data_processor::dto;
use crate::modules::live_data_processor::dto::{DamageDone, HealDone, Message, MessageType, Unit};
use crate::modules::live_data_processor::tools::MapUnit;
use std::collections::HashMap;

/*
 * How is a SpellCast defined?
 * 1. It can have 0-N Damage, Heal and Threat
 * 2. Must have a SpellCast or it is melee damage but then it does not have a spell cast
 * 3. Events are discarded 10ms after the first one was received
 *    OR if the next spell cast or non SpellCast related event is detected
 *    OR if SpellCast events are detected within the next message that don't fit the current SpellCast
 *    OR various other conditions, see below
 */
pub fn try_parse_spell_cast(armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>, non_committed_messages: &Vec<Message>, next_message: &Message, subject: &domain_value::Unit) -> Result<SpellCast, EventParseFailureAction> {
    let mut spell_cast_message = None;
    let mut spell_damage_done_message = Vec::new();
    let mut melee_damage_done_message = Vec::new();
    let mut threat_message = Vec::new();
    let mut heal_message = Vec::new();

    for msg in non_committed_messages.iter() {
        match msg.message_type {
            MessageType::SpellCast(_) => spell_cast_message = Some(msg.clone()),
            MessageType::SpellDamage(_) => spell_damage_done_message.push(msg.clone()),
            MessageType::MeleeDamage(_) => melee_damage_done_message.push(msg.clone()),
            MessageType::Threat(_) => threat_message.push(msg.clone()),
            MessageType::Heal(_) => heal_message.push(msg.clone()),
            _ => {},
        }
    }

    let must_commit =
        // IF timeout is reached
        next_message.timestamp - non_committed_messages.first().unwrap().timestamp > 10
            // IF we already have a spell cast event message and the next is also one
            || (spell_cast_message.is_some() && match &next_message.message_type {
            MessageType::SpellCast(dto::SpellCast { caster, .. }) => caster.to_unit(armory, server_id, summons).contains(subject), // This is expensive! Likewise below
            _ => false
        })
            // IF the next message is non SpellCast related
            || match &next_message.message_type {
            MessageType::SpellCast(_) |
            MessageType::SpellDamage(_) |
            MessageType::MeleeDamage(_) |
            MessageType::Threat(_) |
            MessageType::Heal(_) => false,
            message_type => message_type.extract_subject().map(|unit_dto| unit_dto.to_unit(armory, server_id, summons).contains(subject)).contains(&true)
        }
            // IF the next event is SpellCast related but it seems to be associated with another spell cast
            || !spell_matches(non_committed_messages.first().unwrap(), next_message)
            // IF we collect melee damage but we receive a spell damage events
            || (!melee_damage_done_message.is_empty() && {
            match &next_message.message_type {
                MessageType::SpellCast(dto::SpellCast { caster, .. }) |
                MessageType::SpellDamage(DamageDone { attacker: caster, .. }) |
                MessageType::Heal(HealDone { caster, .. }) => caster.to_unit(armory, server_id, summons).contains(subject),
                _ => false
            }
        })
            // IF we collect spell events but we receive melee events
            || ((spell_cast_message.is_some() || !spell_damage_done_message.is_empty() || !heal_message.is_empty())
            && match &next_message.message_type {
            MessageType::MeleeDamage(DamageDone { attacker: caster, .. }) => caster.to_unit(armory, server_id, summons).contains(subject),
            _ => false
        });

    // We only commit if we must commit, else we wait for events that are associated with this cast
    if !must_commit {
        return Err(EventParseFailureAction::Wait);
    }

    // If we still don't have a spell cast or no melee damage events, we must discard all events!
    if spell_cast_message.is_none() && melee_damage_done_message.is_empty() {
        return Err(EventParseFailureAction::DiscardAll);
    }

    let mut damage = spell_damage_done_message
        .into_iter()
        .map(|message| parse_spell_damage_message(message, armory, server_id, summons))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect::<Vec<Damage>>();
    damage.append(
        melee_damage_done_message
            .into_iter()
            .map(|message| parse_melee_damage_message(message, armory, server_id, summons))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect::<Vec<Damage>>()
            .as_mut(),
    );
    let heal = heal_message
        .into_iter()
        .map(|message| parse_heal_message(message, armory, server_id, summons))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect::<Vec<Heal>>();
    let threat = threat_message
        .into_iter()
        .map(|message| parse_threat_message(message, armory, server_id, summons))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect::<Vec<Threat>>();

    if let Some(spell_message) = spell_cast_message {
        if let MessageType::SpellCast(dto::SpellCast { target, spell_id, hit_type, .. }) = spell_message.message_type {
            return Ok(SpellCast {
                victim: target.and_then(|unit| unit.to_unit(armory, server_id, summons).ok()),
                hit_type: HitType::from_u8(hit_type),
                spell_id: Some(spell_id),
                damage,
                heal,
                threat,
            });
        }
    }
    Ok(SpellCast {
        victim: damage.first().map(|damage| damage.victim.clone()),
        hit_type: damage.first().map(|damage| damage.hit_type.clone()).expect("Damage done must have at least one entry"),
        spell_id: None,
        damage,
        heal,
        threat,
    })
}

fn parse_spell_damage_message(spell_damage_message: Message, armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Option<Damage> {
    if let MessageType::SpellDamage(spell_damage) = spell_damage_message.message_type {
        let mut mitigation = Vec::new();
        if spell_damage.blocked > 0 {
            mitigation.push(Mitigation::Block(spell_damage.blocked));
        }
        if spell_damage.absorbed > 0 {
            mitigation.push(Mitigation::Absorb(spell_damage.absorbed));
        }
        if spell_damage.resisted_or_glanced > 0 {
            mitigation.push(Mitigation::Resist(spell_damage.resisted_or_glanced));
        }

        return Some(Damage {
            school: School::from_u8(spell_damage.school),
            damage: spell_damage.damage,
            mitigation,
            victim: spell_damage.victim.to_unit(armory, server_id, summons).ok()?,
            // TODO: How to tell its a crit?
            hit_type: HitType::Hit,
        });
    }
    panic!("This method should only receive spell damage message types")
}

fn parse_melee_damage_message(melee_damage_message: Message, armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Option<Damage> {
    if let MessageType::MeleeDamage(melee_damage) = melee_damage_message.message_type {
        let mut mitigation = Vec::new();
        if melee_damage.blocked > 0 {
            mitigation.push(Mitigation::Block(melee_damage.blocked));
        }
        if melee_damage.absorbed > 0 {
            mitigation.push(Mitigation::Absorb(melee_damage.absorbed));
        }
        if melee_damage.resisted_or_glanced > 0 {
            mitigation.push(Mitigation::Glance(melee_damage.resisted_or_glanced));
        }

        return Some(Damage {
            school: School::from_u8(melee_damage.school),
            damage: melee_damage.damage,
            mitigation,
            victim: melee_damage.victim.to_unit(armory, server_id, summons).ok()?,
            hit_type: HitType::from_u8(melee_damage.hit_type.expect("TODO: Melee Damage Hit Type can be None?")),
        });
    }
    panic!("This method should only receive melee damage message types")
}

fn parse_heal_message(heal_message: Message, armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Option<Heal> {
    if let MessageType::Heal(heal_done) = heal_message.message_type {
        return Some(Heal {
            total: heal_done.total_heal,
            effective: heal_done.effective_heal,
            mitigation: if heal_done.absorb > 0 { vec![Mitigation::Absorb(heal_done.absorb)] } else { Vec::new() },
            target: heal_done.target.to_unit(armory, server_id, summons).ok()?,
        });
    }
    panic!("This method should only receive heal message types")
}

fn parse_threat_message(threat_message: Message, armory: &Armory, server_id: u32, summons: &HashMap<u64, u64>) -> Option<Threat> {
    if let MessageType::Threat(threat) = threat_message.message_type {
        return if let Ok(threatened @ domain_value::Unit::Creature(_)) = threat.threatened.to_unit(armory, server_id, summons) {
            Some(Threat { threatened, amount: threat.amount })
        } else {
            None
        };
    }
    panic!("This method should only receive threat message types")
}

fn extract_spell_attacker(message_type: &MessageType) -> Option<Unit> {
    match message_type {
        MessageType::MeleeDamage(item) => Some(item.attacker.clone()),
        MessageType::SpellDamage(item) => Some(item.attacker.clone()),
        MessageType::Heal(item) => Some(item.caster.clone()),
        MessageType::SpellCast(item) => Some(item.caster.clone()),
        MessageType::Threat(item) => Some(item.threater.clone()),
        _ => None,
    }
}

fn extract_spell_target(message_type: &MessageType) -> Option<Unit> {
    match message_type {
        MessageType::MeleeDamage(item) => Some(item.victim.clone()),
        MessageType::SpellDamage(item) => Some(item.victim.clone()),
        MessageType::Heal(item) => Some(item.target.clone()),
        MessageType::SpellCast(item) => item.target.clone(),
        MessageType::Threat(item) => Some(item.threatened.clone()),
        _ => None,
    }
}

fn extract_spell_id(message_type: &MessageType) -> Option<u32> {
    match message_type {
        MessageType::MeleeDamage(item) => item.spell_id,
        MessageType::SpellDamage(item) => item.spell_id,
        MessageType::Heal(item) => Some(item.spell_id),
        MessageType::SpellCast(item) => Some(item.spell_id),
        MessageType::Threat(item) => item.spell_id,
        _ => None,
    }
}

fn spell_matches(msg1: &Message, msg2: &Message) -> bool {
    let condition = extract_spell_attacker(&msg2.message_type) == extract_spell_attacker(&msg1.message_type) && extract_spell_id(&msg2.message_type) == extract_spell_id(&msg1.message_type);

    // For threat the threatened may be a third unit
    if let MessageType::Threat(_) = msg1.message_type {
        return condition;
    } else if let MessageType::Threat(_) = msg2.message_type {
        return condition;
    }
    condition && extract_spell_target(&msg2.message_type) == extract_spell_target(&msg1.message_type)
}
