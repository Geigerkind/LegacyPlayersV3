use crate::modules::live_data_processor::domain_value::{HitMask, Unit};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpellCast {
    pub victim: Option<Unit>,
    pub hit_mask: HitMask,
    pub spell_id: u32,
}
