#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct SpellCost {
  cost: u32,
  power_type: String
}