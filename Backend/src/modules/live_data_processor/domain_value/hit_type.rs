#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum HitType {
  Evade,
  Miss,
  Dodge,
  Block,
  Parry,
  Glancing,
  Crit,
  Crushing,
  Hit,
  Resist,
  Immune,
  Environment,
  Absorb,
  Interrupted
}