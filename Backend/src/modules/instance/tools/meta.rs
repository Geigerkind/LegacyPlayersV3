use crate::modules::instance::domain_value::{InstanceMeta, PrivacyType};
use crate::modules::instance::dto::{InstanceFailure, InstancePrivacy};
use crate::modules::instance::Instance;
use crate::params;
use crate::util::database::Execute;

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

pub trait UpdateMeta {
    fn update_privacy(&self, db_main: &mut impl Execute, instance_privacy: InstancePrivacy, member_id: u32) -> Result<(), InstanceFailure>;
}

impl UpdateMeta for Instance {
    fn update_privacy(&self, db_main: &mut impl Execute, instance_privacy: InstancePrivacy, member_id: u32) -> Result<(), InstanceFailure> {
        if db_main.execute_wparams("UPDATE `instance_meta` A \
        JOIN `instance_uploads` B ON A.upload_id = B.id \
        SET `privacy_type`=:privacy_type, `privacy_ref`=:privacy_ref \
        WHERE B.member_id=:member_id AND A.id=:instance_meta_id", params!("member_id" => member_id, "instance_meta_id" => instance_privacy.instance_meta_id,
        "privacy_type" => instance_privacy.privacy_option, "privacy_ref" => instance_privacy.privacy_group)) {
            let mut instance_metas = self.instance_metas.write().unwrap();
            let mut instance_meta = instance_metas.1.get_mut(&instance_privacy.instance_meta_id).ok_or(InstanceFailure::InvalidInput)?;
            instance_meta.privacy_type = PrivacyType::new(instance_privacy.privacy_option, instance_privacy.privacy_group);
            return Ok(());
        }
        Err(InstanceFailure::Unknown)
    }
}
