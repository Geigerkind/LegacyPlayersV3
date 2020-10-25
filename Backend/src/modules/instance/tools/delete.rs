use crate::modules::instance::Instance;
use crate::util::database::{Execute, Select};
use crate::modules::instance::dto::InstanceFailure;
use crate::params;
use crate::modules::armory::Armory;
use std::fs;

pub trait DeleteInstance {
    fn delete_instance(&self, db_main: &mut (impl Execute + Select), armory: &Armory, instance_meta_id: u32, member_id: u32) -> Result<(), InstanceFailure>;
}

impl DeleteInstance for Instance {
    fn delete_instance(&self, db_main: &mut (impl Execute + Select), armory: &Armory, instance_meta_id: u32, member_id: u32) -> Result<(), InstanceFailure> {
        let storage_path = std::env::var("INSTANCE_STORAGE_PATH").expect("storage path must be set");
        let server_id: Option<u32> = db_main.select_wparams_value("SELECT server_id FROM instance_meta WHERE id=:instance_meta_id",
                                                                  |mut row| row.take::<u32, usize>(0).unwrap(),
                                                                  params!("instance_meta_id" => instance_meta_id));
        if let Some(server_id) = server_id {
            if db_main.execute_wparams("DELETE FROM instance_meta WHERE id=:instance_meta_id AND uploaded_user=:member_id", params!(
            "instance_meta_id" => instance_meta_id,
            "member_id" => member_id
        )) {
                self.update_instance_meta(db_main, armory);
                let _ = fs::remove_dir_all(&format!("{}/{}/{}", storage_path, server_id, instance_meta_id));
                return Ok(());
            }
        }
        Err(InstanceFailure::InvalidInput)
    }
}