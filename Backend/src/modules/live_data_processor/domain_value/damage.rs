use crate::modules::live_data_processor::domain_value::{HitMask, Mitigation, SchoolMask, Unit};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Damage {
    #[serde(skip)]
    pub school_mask: SchoolMask,
    pub damage: u32,
    pub mitigation: Vec<Mitigation>,
    pub victim: Unit,
    pub hit_mask: HitMask,
}
