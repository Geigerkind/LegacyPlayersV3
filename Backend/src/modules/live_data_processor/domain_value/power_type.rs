#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum PowerType {
  Mana,
  Rage,
  Focus,
  Energy,
  Happiness,
  Health
}