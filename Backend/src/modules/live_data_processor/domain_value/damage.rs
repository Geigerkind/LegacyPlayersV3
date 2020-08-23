use crate::modules::live_data_processor::domain_value::{SpellComponent, HitMask, Unit};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Damage {
    pub victim: Unit,
    pub hit_mask: HitMask,
    pub components: Vec<SpellComponent>,
}
