use crate::modules::tooltip::domain_value::SpellCost;

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct SpellTooltip {
  name: String,
  icon: String,
  subtext: Option<String>,
  spell_cost: Option<SpellCost>,
  range: u32,
  description: String
}