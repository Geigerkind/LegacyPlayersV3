use crate::modules::armory::domain_value::{CharacterInfo, CharacterGuild};
use crate::modules::armory::dto::CharacterHistoryDto;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterHistory {
  pub id: u32,
  pub character_id: u32,
  pub character_info: CharacterInfo,
  pub character_name: String,
  pub guild: Option<CharacterGuild>,
  pub timestamp: u64
}

impl PartialEq for CharacterHistory {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
  fn ne(&self, other: &Self) -> bool {
    self.id != other.id
  }
}

impl CharacterHistory {
  pub fn deep_eq(&self, other: &Self) -> bool {
    self.id == other.id
      && self.character_id == other.character_id
      && self.character_info.deep_eq(&other.character_info)
      && self.character_name == other.character_name
      && ((self.guild.is_none() && other.guild.is_none())
          || (self.guild.is_some() && other.guild.is_some() && self.guild.as_ref().unwrap().deep_eq(other.guild.as_ref().unwrap())))
      && self.timestamp == other.timestamp
  }

  pub fn compare_by_value(&self, other: &CharacterHistoryDto) -> bool {
    self.character_info.compare_by_value(&other.character_info)
      && self.character_name == other.character_name
      && ((self.guild.is_none() && other.guild.is_none())
          || (self.guild.is_some() && other.guild.is_some()
    )         && self.guild.as_ref().unwrap().compare_by_value(other.guild.as_ref().unwrap()))
    // Technically we should also compare character_id => character_uid and guild_id => guild_dto
    // But this would require to make a get call
  }
}