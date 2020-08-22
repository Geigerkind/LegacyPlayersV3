use crate::modules::live_data_processor::dto::Unit;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct DamageDone {
    pub attacker: Unit,
    pub victim: Unit,
    pub spell_id: Option<u32>,
    pub hit_mask: u32,
    pub blocked: u32,
    pub school_mask: u8,
    pub damage: u32,
    pub resisted_or_glanced: u32,
    pub absorbed: u32,
    pub damage_over_time: bool,
}
