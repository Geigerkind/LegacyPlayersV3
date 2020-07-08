use crate::modules::live_data_processor::domain_value::{AuraApplication, Position, Power, SpellCast, Unit};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum EventType {
    SpellCast(SpellCast),
    Death { murder: Option<Unit> },
    CombatState { in_combat: bool },
    Loot { item_id: u32 },
    Position(Position),
    Power(Power),
    AuraApplication(AuraApplication),
    Interrupt { cause_event_id: u32, interrupted_spell_id: u32 },
    SpellSteal { cause_event_id: u32, target_event_id: u32 },
    Dispel { cause_event_id: u32, target_event_ids: Vec<u32> },
    ThreatWipe { creature: Unit },
    Summon { summoned: Unit },

    // Used for convenience
    PlaceHolder,
}
