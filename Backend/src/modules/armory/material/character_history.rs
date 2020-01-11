use crate::modules::armory::domain_value::CharacterInfo;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterHistory {
  pub id: u32,
  pub character_id: u32,
  pub character_info: CharacterInfo,
  pub character_name: String,
  pub guild_id: Option<u32>,
  pub guild_rank: Option<String>,
  pub timestamp: u64
}