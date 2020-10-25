use crate::modules::instance::Instance;
use crate::util::database::{Execute, Select};
use crate::modules::instance::dto::InstanceFailure;
use crate::params;
use crate::modules::armory::Armory;

pub trait DeleteInstance {
    fn delete_instance(&self, db_main: &mut (impl Execute + Select), armory: &Armory, instance_meta_id: u32, member_id: u32) -> Result<(), InstanceFailure>;
}

impl DeleteInstance for Instance {
    fn delete_instance(&self, db_main: &mut (impl Execute + Select), armory: &Armory, instance_meta_id: u32, member_id: u32) -> Result<(), InstanceFailure> {
        if db_main.execute_wparams("DELETE FROM instance_meta WHERE id=:instance_meta_id AND uploaded_user=:member_id", params!(
            "instance_meta_id" => instance_meta_id,
            "member_id" => member_id
        )) {
            self.update_instance_meta(db_main, armory);
            return Ok(());
        }
        Err(InstanceFailure::InvalidInput)
    }
}