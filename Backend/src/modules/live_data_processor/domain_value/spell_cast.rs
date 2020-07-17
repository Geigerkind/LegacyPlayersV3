use crate::modules::live_data_processor::domain_value::{HitType, Unit};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpellCast {
    pub victim: Option<Unit>,
    pub hit_type: HitType,
    pub spell_id: u32
}
