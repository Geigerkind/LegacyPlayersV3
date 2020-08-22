use crate::modules::live_data_processor::domain_value::{Mitigation, SchoolMask};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DamageComponent {
    pub school_mask: SchoolMask,
    pub damage: u32,
    pub mitigation: Vec<Mitigation>,
}
