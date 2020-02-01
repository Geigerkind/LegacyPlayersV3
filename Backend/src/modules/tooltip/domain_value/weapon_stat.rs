use crate::modules::tooltip::domain_value::WeaponDamage;

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct dWeaponStat {
  pub delay: u16,
  pub damage_sources: Vec<WeaponDamage>
}