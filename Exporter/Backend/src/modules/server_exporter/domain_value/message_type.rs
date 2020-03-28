#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
  MeleeDamage = 0,
  SpellDamage = 1,
  Heal = 2,
  Death = 3,
  AuraApplication = 4,
  Dispel = 5,
  SpellSteal = 6,
  Interrupt = 7,
  Position = 8,
  CombatState = 9,
  Power = 10,
  Loot = 11,
  SpellCast = 12,
  Threat = 13,
  Event = 14,
  Summon = 15,
  InstancePvpStart = 16,
  InstancePvpEndUnratedArena = 17,
  InstancePvpEndRatedArena = 18,
  InstancePvpEndBattleground = 19,
  Undefined = 255
}

impl MessageType {
  pub fn from_number(number: &u8) -> Self {
    match number {
      0 => MessageType::MeleeDamage,
      1 => MessageType::SpellDamage,
      2 => MessageType::Heal,
      3 => MessageType::Death,
      4 => MessageType::AuraApplication,
      5 => MessageType::Dispel,
      6 => MessageType::SpellSteal,
      7 => MessageType::Interrupt,
      8 => MessageType::Position,
      9 => MessageType::CombatState,
      10 => MessageType::Power,
      11 => MessageType::Loot,
      12 => MessageType::SpellCast,
      13 => MessageType::Threat,
      14 => MessageType::Event,
      15 => MessageType::Summon,
      16 => MessageType::InstancePvpStart,
      17 => MessageType::InstancePvpEndUnratedArena,
      18 => MessageType::InstancePvpEndRatedArena,
      19 => MessageType::InstancePvpEndBattleground,
      _ => MessageType::Undefined
    }
  }
}