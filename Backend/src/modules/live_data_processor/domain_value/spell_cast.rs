use crate::modules::live_data_processor::domain_value::{Unit, HitType, Damage, Heal, Threat};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpellCast {
  pub victim: Option<Unit>,
  pub hit_type: HitType,
  pub spell_id: Option<u32>,
  pub damage: Option<Damage>,
  pub heal: Option<Heal>,
  pub threat: Option<Threat>
}