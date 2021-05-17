use crate::modules::instance::domain_value::InstanceMeta;
use crate::modules::instance::Instance;

pub trait ExportMeta {
    fn export_meta(&self, meta_type: u8) -> Vec<InstanceMeta>;
}

impl ExportMeta for Instance {
    fn export_meta(&self, meta_type: u8) -> Vec<InstanceMeta> {
        let instance_metas = self.instance_metas.read().unwrap();
        instance_metas.1
            .iter()
            .filter(|(_instance_meta_id, instance_meta)| instance_meta.instance_specific.to_u8() == meta_type)
            .map(|(_, instance_meta)| instance_meta.clone())
            .collect()
    }
}
