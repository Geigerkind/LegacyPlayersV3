use crate::modules::instance::domain_value::InstanceMeta;
use crate::modules::instance::dto::InstanceFailure;
use crate::modules::instance::Instance;

pub trait ExportMeta {
    fn export_meta(&self, meta_type: u8) -> Result<Vec<InstanceMeta>, InstanceFailure>;
}

impl ExportMeta for Instance {
    fn export_meta(&self, _meta_type: u8) -> Result<Vec<InstanceMeta>, InstanceFailure> {
        unimplemented!()
    }
}
