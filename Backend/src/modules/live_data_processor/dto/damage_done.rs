use crate::modules::live_data_processor::dto::Unit;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct DamageDone {
    pub attacker: Unit,
    pub victim: Unit,
    pub spell_id: Option<u32>,
    pub hit_type: Option<u8>,
    pub blocked: u32,
    pub school: u8,
    pub damage: u32,
    pub resisted_or_glanced: u32,
    pub absorbed: u32,
}
