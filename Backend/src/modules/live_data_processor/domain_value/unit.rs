use crate::modules::live_data_processor::domain_value::{Creature, Player};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum Unit {
    Player(Player),
    Creature(Creature),
}
