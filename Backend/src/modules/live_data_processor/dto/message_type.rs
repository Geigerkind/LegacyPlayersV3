use crate::modules::live_data_processor::dto::{AuraApplication, CombatState, DamageDone, Death, Event, HealDone, Instance, InstanceArena, InstanceBattleground, Interrupt, Loot, Position, Power, SpellCast, Summon, Threat, UnAura};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum MessageType {
    MeleeDamage(DamageDone),
    SpellDamage(DamageDone),
    Heal(HealDone),
    Death(Death),
    AuraApplication(AuraApplication),
    Dispel(UnAura),
    SpellSteal(UnAura),
    Interrupt(Interrupt),
    Position(Position),
    CombatState(CombatState),
    Power(Power),
    Loot(Loot),
    SpellCast(SpellCast),
    Threat(Threat),
    Event(Event),
    Summon(Summon),
    InstancePvPStart(Instance),
    InstancePvPEndUnratedArena(Instance),
    InstancePvPEndRatedArena(InstanceArena),
    InstancePvPEndBattleground(InstanceBattleground),
}
