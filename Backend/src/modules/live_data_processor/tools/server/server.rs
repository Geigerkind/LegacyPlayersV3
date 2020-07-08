use crate::modules::armory::Armory;
use crate::modules::live_data_processor::domain_value;
use crate::modules::live_data_processor::domain_value::{AuraApplication, Event, EventParseFailureAction, EventType, Position, Power, PowerType, UnitInstance};
use crate::modules::live_data_processor::dto::{CombatState, DamageDone, Death, HealDone, Loot, SpellCast, Summon, Threat};
use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Message, MessageType};
use crate::modules::live_data_processor::material::Server;
use crate::modules::live_data_processor::tools::server::{try_parse_dispel, try_parse_interrupt, try_parse_spell_cast, try_parse_spell_steal};
use crate::modules::live_data_processor::tools::MapUnit;

impl Server {
    pub fn parse_events(&mut self, armory: &Armory, messages: Vec<Message>) -> Result<(), LiveDataProcessorFailure> {
        for msg in messages {
            self.extract_meta_information(&msg);
            self.test_for_committable_events(armory, &msg);
            self.cleanup(msg.timestamp);
            self.push_non_committed_event(msg);
        }
        Ok(())
    }

    fn push_non_committed_event(&mut self, message: Message) {
        // TODO: What do we do with non unit associated events?
        if let Some(unit_dto) = message.message_type.extract_subject() {
            if let Some(event) = self.non_committed_events.get_mut(&unit_dto.unit_id) {
                event.push(message);
            } else {
                self.non_committed_events.insert(unit_dto.unit_id, vec![message]);
            }
        }
    }

    fn test_for_committable_events(&mut self, armory: &Armory, next_message: &Message) {
        let mut remove_all_non_committed_events = Vec::new();
        let mut remove_first_non_committed_event = Vec::new();
        for (subject_id, non_committed_event) in self.non_committed_events.iter() {
            match self.commit_event(armory, non_committed_event, next_message) {
                Ok(mut committable_event) => {
                    committable_event.id = (self.committed_events.len() + 1) as u32;

                    // For all except Spell we want to only remove the first event
                    match &committable_event {
                        Event { event: EventType::SpellCast(_), .. } => {
                            remove_all_non_committed_events.push(*subject_id);
                        },
                        _ => {
                            remove_first_non_committed_event.push(*subject_id);
                        },
                    };

                    self.committed_events.push(committable_event);
                },
                Err(EventParseFailureAction::DiscardAll) => {
                    remove_all_non_committed_events.push(*subject_id);
                },
                Err(EventParseFailureAction::DiscardFirst) => {
                    remove_first_non_committed_event.push(*subject_id);
                },
                Err(EventParseFailureAction::Wait) => {},
            };
        }
        for subject_id in remove_all_non_committed_events {
            self.non_committed_events.remove(&subject_id);
        }

        for subject_id in remove_first_non_committed_event {
            self.non_committed_events.get_mut(&subject_id).expect("subject id should exist").pop();
        }
    }

    fn cleanup(&mut self, current_timestamp: u64) {
        for subject_id in self
            .non_committed_events
            .iter()
            .filter(|(_subject_id, event)| event.first().expect("Should be initialized with at least one element").timestamp + 10 < current_timestamp)
            .map(|(subject_id, _event)| *subject_id)
            .collect::<Vec<u64>>()
        {
            self.non_committed_events.remove(&subject_id);
        }
    }

    // So based on the next event for the current users in the system
    // we are going to decide whether or not to commit it.
    fn commit_event(&self, armory: &Armory, non_committed_event: &Vec<Message>, next_message: &Message) -> Result<Event, EventParseFailureAction> {
        let first_message = non_committed_event.first().expect("non_committed_event contains at least one entry");
        match &first_message.message_type {
            // Events that are just of size 1
            MessageType::CombatState(CombatState { unit: unit_dto, in_combat }) => Ok(Event::new(
                first_message.timestamp,
                unit_dto.to_unit(armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                EventType::CombatState { in_combat: *in_combat },
            )),
            MessageType::Loot(Loot { unit: unit_dto, item_id }) => Ok(Event::new(
                first_message.timestamp,
                unit_dto.to_unit(armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                EventType::Loot { item_id: *item_id },
            )),
            MessageType::Position(position) => Ok(Event::new(
                first_message.timestamp,
                position.unit.to_unit(armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                EventType::Position((
                    UnitInstance {
                        map_id: position.map_id,
                        map_difficulty: position.map_difficulty,
                        instance_id: position.instance_id,
                    },
                    Position {
                        x: position.x,
                        y: position.y,
                        z: position.z,
                        orientation: position.orientation,
                    },
                )),
            )),
            MessageType::Power(power) => Ok(Event::new(
                first_message.timestamp,
                power.unit.to_unit(armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                EventType::Power(Power {
                    power_type: PowerType::from_u8(power.power_type).ok_or_else(|| EventParseFailureAction::DiscardFirst)?,
                    max_power: power.max_power,
                    current_power: power.current_power,
                }),
            )),
            MessageType::AuraApplication(aura_application) => Ok(Event::new(
                first_message.timestamp,
                aura_application.target.to_unit(armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                EventType::AuraApplication(AuraApplication {
                    // TODO: This can also be an object, do we support this?
                    caster: aura_application.caster.to_unit(armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                    stack_amount: aura_application.stack_amount,
                    spell_id: aura_application.spell_id,
                }),
            )),
            MessageType::Death(Death { cause, victim }) => Ok(Event::new(
                first_message.timestamp,
                victim.to_unit(armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?,
                EventType::Death {
                    murder: cause.as_ref().and_then(|cause| cause.to_unit(&armory, self.server_id, &self.summons).ok()),
                },
            )),
            MessageType::Event(event_dto) => {
                if event_dto.event_type == 0 {
                    if let Ok(creature @ domain_value::Unit::Creature(_)) = event_dto.unit.to_unit(armory, self.server_id, &self.summons) {
                        // TODO: Is the creature really the unit that we want to return here?
                        return Ok(Event::new(first_message.timestamp, creature.clone(), EventType::ThreatWipe { creature }));
                    }
                }
                Err(EventParseFailureAction::DiscardFirst)
            },
            // TODO: This can be an event
            /*
            MessageType::Summon(Summon { owner, unit }) => {
                self.summons.insert(owner.unit_id, unit.unit_id);
                None
            },*/
            // Spell can be between 1 and N events
            MessageType::SpellCast(SpellCast { caster: unit, .. })
            | MessageType::Threat(Threat { threater: unit, .. })
            | MessageType::Heal(HealDone { caster: unit, .. })
            | MessageType::MeleeDamage(DamageDone { attacker: unit, .. })
            | MessageType::SpellDamage(DamageDone { attacker: unit, .. }) => {
                let subject = unit.to_unit(armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardAll)?;
                Ok(Event::new(
                    first_message.timestamp,
                    subject.clone(),
                    EventType::SpellCast(try_parse_spell_cast(armory, self.server_id, &self.summons, &non_committed_event, &next_message, &subject)?),
                ))
            },

            // Find Event that caused this interrupt, else wait or discard
            MessageType::Interrupt(interrupt) => {
                let subject = interrupt.target.to_unit(armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                match try_parse_interrupt(&interrupt, &self.committed_events, first_message.timestamp, &subject) {
                    Ok((cause_event_id, interrupted_spell_id)) => Ok(Event::new(first_message.timestamp, subject, EventType::Interrupt { cause_event_id, interrupted_spell_id })),
                    Err(err) => Err(err),
                }
            },
            // Find Event that caused this dispel, else wait or discard
            MessageType::Dispel(dispel) => {
                let subject = dispel.aura_caster.to_unit(armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                match try_parse_dispel(&dispel, &self.committed_events, first_message.timestamp, next_message.timestamp, armory, self.server_id, &self.summons) {
                    Ok((cause_event_id, target_event_ids)) => Ok(Event::new(first_message.timestamp, subject, EventType::Dispel { cause_event_id, target_event_ids })),
                    Err(err) => Err(err),
                }
            },
            // Find Event that caused this spell steal, else wait or discard
            MessageType::SpellSteal(spell_steal) => {
                let subject = spell_steal.aura_caster.to_unit(armory, self.server_id, &self.summons).map_err(|_| EventParseFailureAction::DiscardFirst)?;
                match try_parse_spell_steal(&spell_steal, &self.committed_events, first_message.timestamp, next_message.timestamp, armory, self.server_id, &self.summons) {
                    Ok((cause_event_id, target_event_id)) => Ok(Event::new(first_message.timestamp, subject, EventType::SpellSteal { cause_event_id, target_event_id })),
                    Err(err) => Err(err),
                }
            },

            // Not handled yet
            /*
            MessageType::InstancePvPStart(_) | MessageType::InstancePvPEndUnratedArena(_) | MessageType::InstancePvPEndBattleground(_) | MessageType::InstancePvPEndRatedArena(_) |
             */
            _ => Err(EventParseFailureAction::DiscardFirst),
        }
    }

    fn extract_meta_information(&mut self, message: &Message) {
        if let MessageType::Summon(Summon { owner, unit }) = &message.message_type {
            self.summons.insert(owner.unit_id, unit.unit_id);
        }
    }
}