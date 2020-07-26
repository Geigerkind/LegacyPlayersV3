use crate::material::Cachable;
use crate::modules::armory::tools::GetCharacter;
use crate::modules::armory::Armory;
use crate::modules::instance::domain_value::MetaType;
use crate::modules::instance::dto::{InstanceFailure, InstanceViewerGuild, InstanceViewerMeta, InstanceViewerParticipant};
use crate::modules::instance::material::Role;
use crate::modules::instance::tools::FindInstanceGuild;
use crate::modules::instance::Instance;
use crate::modules::live_data_processor::Event;

pub trait ExportInstance {
    fn export_instance_event_type(&self, instance_meta_id: u32, event_type: u8) -> Result<Vec<Event>, InstanceFailure>;
    fn get_instance_meta(&self, armory: &Armory, instance_meta_id: u32) -> Result<InstanceViewerMeta, InstanceFailure>;
    fn get_instance_participants(&self, armory: &Armory, instance_meta_id: u32) -> Result<Vec<InstanceViewerParticipant>, InstanceFailure>;
}

impl ExportInstance for Instance {
    fn export_instance_event_type(&self, instance_meta_id: u32, event_type: u8) -> Result<Vec<Event>, InstanceFailure> {
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
                if (expired.is_some() && expired.unwrap() > last_updated) || (last_updated + 10 > now) {
                    return Ok(cached.get_cached());
                }
            }
        }

        let storage_path = std::env::var("INSTANCE_STORAGE_PATH").expect("storage path must be set");
        let event_path = format!("{}/{}/{}/{}", storage_path, server_id, instance_meta_id, event_type);
        if let Ok(file_content) = std::fs::read_to_string(event_path) {
            let segments = file_content.split('\n').collect::<Vec<&str>>();
            let mut events = Vec::with_capacity(segments.len());
            for segment in segments {
                if segment.len() > 1 {
                    events.push(serde_json::from_str(segment).map_err(|_| InstanceFailure::InvalidInput)?);
                }
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
}
