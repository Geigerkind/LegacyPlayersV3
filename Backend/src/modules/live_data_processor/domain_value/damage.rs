use crate::modules::live_data_processor::domain_value::{DamageComponent, HitMask, Unit};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Damage {
    pub victim: Unit,
    pub hit_mask: HitMask,
    pub damage_components: Vec<DamageComponent>,
}

pub fn get_damage_components_total(components: &Vec<DamageComponent>) -> u32 {
    components.iter().fold(0, |acc, comp| acc + comp.damage)
}
