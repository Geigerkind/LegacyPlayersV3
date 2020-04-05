use crate::modules::live_data_processor::domain_value::{SpellCast, Unit, Position, Power};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum EventType {
  SpellCast(SpellCast),
  Death { murder: Option<Unit> },
  CombatState { in_combat: bool },
  Loot { item_id: u32 },
  Position(Position),
  Power(Power),
  Interrupt { cause: SpellCast, target_event_id: u32 },
  Dispel { cause: SpellCast, target_event_ids: Vec<u32> },
  SpellSteal { cause: SpellCast, target_event_id: u32 },
  AuraApplication { stack_amount: u8, reference_spell_cast: u32 },
  ThreatWipe { creature: Unit },

  // Used for convinience
  PlaceHolder
}