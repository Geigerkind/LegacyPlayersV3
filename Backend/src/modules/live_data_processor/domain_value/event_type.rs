use crate::modules::live_data_processor::domain_value::{AuraApplication, Damage, Heal, Position, Power, SpellCast, Threat, Unit};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum EventType {
    SpellCast(SpellCast),
    Death { murder: Option<Unit> },
    CombatState { in_combat: bool },
    Loot { item_id: u32, amount: u32 },
    Position(Position),
    Power(Power),
    AuraApplication(AuraApplication),
    Interrupt { cause_event_id: u32, interrupted_spell_id: u32 },
    SpellSteal { cause_event_id: u32, target_event_id: u32 },
    Dispel { cause_event_id: u32, target_event_ids: Vec<u32> },
    ThreatWipe,
    Summon { summoned: Unit },
    MeleeDamage(Damage),
    SpellDamage { spell_cause_id: u32, damage: Damage },
    Heal { spell_cause_id: u32, heal: Heal },
    Threat { cause_event_id: u32, threat: Threat },

    // Used for convenience
    PlaceHolder,
}

impl EventType {
    pub fn to_u8(&self) -> u8 {
        match self {
            EventType::SpellCast(_) => 0,
            EventType::Death { .. } => 1,
            EventType::CombatState { .. } => 2,
            EventType::Loot { .. } => 3,
            EventType::Position(_) => 4,
            EventType::Power(_) => 5,
            EventType::AuraApplication(_) => 6,
            EventType::Interrupt { .. } => 7,
            EventType::SpellSteal { .. } => 8,
            EventType::Dispel { .. } => 9,
            EventType::ThreatWipe => 10,
            EventType::Summon { .. } => 11,
            EventType::MeleeDamage { .. } => 12,
            EventType::SpellDamage { .. } => 13,
            EventType::Heal { .. } => 14,
            EventType::Threat { .. } => 15,
            EventType::PlaceHolder => 255,
        }
    }
}
