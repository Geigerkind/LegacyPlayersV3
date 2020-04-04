#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CombatState {
  pub unit: u64,
  pub in_combat: bool
}