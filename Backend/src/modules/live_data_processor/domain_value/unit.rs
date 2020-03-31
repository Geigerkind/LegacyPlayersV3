use crate::modules::live_data_processor::domain_value::{Player, Creature};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Unit {
  Player(Player),
  Creature(Creature)
}