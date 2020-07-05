use crate::modules::live_data_processor::domain_value::{Mitigation, School, Unit, HitType};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Damage {
    pub school: School,
    pub damage: u32,
    pub mitigation: Vec<Mitigation>,
    pub victim: Unit,
    pub hit_type: HitType
}
