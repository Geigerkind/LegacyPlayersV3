use crate::modules::instance::dto::InstanceFailure;
use crate::modules::instance::Instance;
use crate::modules::live_data_processor::Event;

pub trait ExportInstance {
    fn export_instance_event_type(&self, server_id: u32, instance_meta_id: u32, event_type: u8) -> Result<Vec<Event>, InstanceFailure>;
}

impl ExportInstance for Instance {
    fn export_instance_event_type(&self, server_id: u32, instance_meta_id: u32, event_type: u8) -> Result<Vec<Event>, InstanceFailure> {
        let storage_path = std::env::var("INSTANCE_STORAGE_PATH").expect("storage path must be set");
        let event_path = format!("{}/{}/{}/{}", storage_path, server_id, instance_meta_id, event_type);
        if let Ok(file_content) = std::fs::read_to_string(event_path) {
            let segments = file_content.split('\n').collect::<Vec<&str>>();
            let mut events = Vec::with_capacity(segments.len());
            for segment in segments {
                if segment.len() > 1 {
                    println!("{}", segment);
                    events.push(serde_json::from_str(segment).map_err(|_| InstanceFailure::InvalidInput)?);
                }
            }
            return Ok(events);
        }
        Err(InstanceFailure::InvalidInput)
    }
}
