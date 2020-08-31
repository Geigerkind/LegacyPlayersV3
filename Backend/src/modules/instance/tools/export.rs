use crate::material::Cachable;
use crate::modules::armory::tools::GetCharacter;
use crate::modules::armory::Armory;
use crate::modules::instance::domain_value::MetaType;
use crate::modules::instance::dto::{InstanceFailure, InstanceViewerAttempt, InstanceViewerGuild, InstanceViewerMeta, InstanceViewerParticipant};
use crate::modules::instance::material::Role;
use crate::modules::instance::tools::FindInstanceGuild;
use crate::modules::instance::Instance;
use crate::params;
use crate::util::database::Select;
use std::str::FromStr;

pub trait ExportInstance {
    fn export_instance_event_type(&self, instance_meta_id: u32, event_type: u8) -> Result<Vec<(u32, String)>, InstanceFailure>;
    fn get_instance_meta(&self, armory: &Armory, instance_meta_id: u32) -> Result<InstanceViewerMeta, InstanceFailure>;
    fn get_instance_participants(&self, armory: &Armory, instance_meta_id: u32) -> Result<Vec<InstanceViewerParticipant>, InstanceFailure>;
    fn get_instance_attempts(&self, db_main: &mut impl Select, instance_meta_id: u32) -> Result<Vec<InstanceViewerAttempt>, InstanceFailure>;
}

impl ExportInstance for Instance {
    fn export_instance_event_type(&self, instance_meta_id: u32, event_type: u8) -> Result<Vec<(u32, String)>, InstanceFailure> {
        let (server_id, expired) = {
            let instance_metas = self.instance_metas.read().unwrap();
            let instance_meta = instance_metas.get(&instance_meta_id).ok_or_else(|| InstanceFailure::InvalidInput)?;
            (instance_meta.server_id, instance_meta.expired)
        };

        {
            let instance_exports = self.instance_exports.read().unwrap();
            if let Some(cached) = instance_exports.get(&(instance_meta_id, event_type)) {
                let now = time_util::now();
                let last_updated = cached.get_last_updated();
                if (expired.is_some() && expired.unwrap() < last_updated) || (last_updated + 10 > now) {
                    return Ok(cached.get_cached());
                }
            }
        }

        let storage_path = std::env::var("INSTANCE_STORAGE_PATH").expect("storage path must be set");
        let event_path = format!("{}/{}/{}/{}", storage_path, server_id, instance_meta_id, event_type);
        if let Ok(file_content) = std::fs::read_to_string(event_path) {
            let lines = file_content.lines().collect::<Vec<&str>>();
            let mut events = Vec::with_capacity(lines.len());
            for segment in lines {
                let id = u32::from_str(&segment[1..segment.find(',').expect("Must exist if data is not broken")])
                    .expect("First element is the id");
                events.push((id, segment.to_owned()));
            }

            let mut instance_exports = self.instance_exports.write().unwrap();
            instance_exports.insert((instance_meta_id, event_type), Cachable::new(events.clone()));

            return Ok(events);
        }
        Ok(vec![])
    }

    fn get_instance_meta(&self, armory: &Armory, instance_meta_id: u32) -> Result<InstanceViewerMeta, InstanceFailure> {
        let instance_metas = self.instance_metas.read().unwrap();
        if let Some(instance_meta) = instance_metas.get(&instance_meta_id) {
            let guild = instance_meta.participants.find_instance_guild(armory);
            let map_difficulty = match instance_meta.instance_specific {
                MetaType::Raid { map_difficulty } => Some(map_difficulty),
                _ => None,
            };
            return Ok(InstanceViewerMeta {
                instance_meta_id,
                guild: guild.map(|guild| InstanceViewerGuild { guild_id: guild.id, guild_name: guild.name }),
                server_id: instance_meta.server_id,
                map_id: instance_meta.map_id,
                map_difficulty,
                start_ts: instance_meta.start_ts,
                end_ts: instance_meta.end_ts,
                expired: instance_meta.expired,
            });
        }
        Err(InstanceFailure::InvalidInput)
    }

    fn get_instance_participants(&self, armory: &Armory, instance_meta_id: u32) -> Result<Vec<InstanceViewerParticipant>, InstanceFailure> {
        let instance_metas = self.instance_metas.read().unwrap();
        if let Some(instance_meta) = instance_metas.get(&instance_meta_id) {
            return Ok(instance_meta
                .participants
                .iter()
                .map(|character_id| armory.get_character(*character_id))
                .filter(|character| character.is_some())
                .map(|character| character.unwrap())
                .map(|character| {
                    // TODO: Use a character at a time of the raid
                    let hero_class_id = character.last_update.as_ref().map_or_else(|| 1, |history| history.character_info.hero_class_id);
                    InstanceViewerParticipant {
                        character_id: character.id,
                        name: character.last_update.as_ref().map_or_else(|| String::from("Unknown"), |history| history.character_name.clone()),
                        hero_class_id,
                        role: Role::from_class_talent_string(
                            hero_class_id,
                            &character.last_update.map_or_else(|| String::from(""), |history| history.character_info.talent_specialization.unwrap_or_else(|| String::from(""))),
                        ),
                    }
                })
                .collect());
        }
        Err(InstanceFailure::InvalidInput)
    }

    fn get_instance_attempts(&self, db_main: &mut impl Select, instance_meta_id: u32) -> Result<Vec<InstanceViewerAttempt>, InstanceFailure> {
        let expired = {
            let instance_metas = self.instance_metas.read().unwrap();
            let instance_meta = instance_metas.get(&instance_meta_id).ok_or_else(|| InstanceFailure::InvalidInput)?;
            instance_meta.expired
        };

        {
            let instance_attempts = self.instance_attempts.read().unwrap();
            if let Some(cached) = instance_attempts.get(&instance_meta_id) {
                let now = time_util::now();
                let last_updated = cached.get_last_updated();
                if (expired.is_some() && expired.unwrap() < last_updated) || (last_updated + 10 > now) {
                    return Ok(cached.get_cached());
                }
            }
        }

        let attempts: Vec<InstanceViewerAttempt> = db_main
            .select_wparams(
                "SELECT id, encounter_id, start_ts, end_ts, is_kill FROM `instance_attempt` WHERE instance_meta_id=:instance_meta_id",
                |mut row| InstanceViewerAttempt {
                    id: row.take(0).unwrap(),
                    encounter_id: row.take(1).unwrap(),
                    start_ts: row.take(2).unwrap(),
                    end_ts: row.take(3).unwrap(),
                    is_kill: row.take(4).unwrap(),
                },
                params!("instance_meta_id" => instance_meta_id),
            )
            .into_iter()
            .collect();

        let mut instance_attempts = self.instance_attempts.write().unwrap();
        instance_attempts.insert(instance_meta_id, Cachable::new(attempts.clone()));
        Ok(attempts)
    }
}
