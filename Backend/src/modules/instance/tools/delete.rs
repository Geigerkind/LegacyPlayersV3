use crate::modules::instance::Instance;
use crate::util::database::Execute;

pub trait DeleteInstance {
    fn delete_instance(&self, db_main: &mut impl Execute, member_id: u32);
}

impl DeleteInstance for Instance {
    fn delete_instance(&self, db_main: &mut impl Execute, member_id: u32) {
        unimplemented!()
    }
}