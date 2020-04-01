use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;

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
  InstancePvPStart = 16,
  InstancePvPEndUnratedArena = 17,
  InstancePvPEndRatedArena = 18,
  InstancePvPEndBattleground = 19
}

pub trait ParseMessageType {
  fn to_message_type(&self) -> Result<MessageType, LiveDataProcessorFailure>;
}

impl ParseMessageType for u8 {
  fn to_message_type(&self) -> Result<MessageType, LiveDataProcessorFailure> {
    Ok(match self {
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
      16 => MessageType::InstancePvPStart,
      17 => MessageType::InstancePvPEndUnratedArena,
      18 => MessageType::InstancePvPEndRatedArena,
      19 => MessageType::InstancePvPEndBattleground,
      _ => return Err(LiveDataProcessorFailure::InvalidInput)
    })
  }
}