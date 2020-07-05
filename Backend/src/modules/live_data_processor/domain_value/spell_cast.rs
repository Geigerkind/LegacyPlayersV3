use crate::modules::live_data_processor::domain_value::{Damage, Heal, HitType, Threat, Unit};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpellCast {
    pub victim: Option<Unit>,
    pub hit_type: HitType,
    pub spell_id: Option<u32>,
    pub damage: Vec<Damage>,
    pub heal: Vec<Heal>,
    pub threat: Vec<Threat>,
}
