use crate::modules::armory::domain_value::CharacterInfo;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterHistoryDto {
  pub character_uid: u64,
  pub character_info: CharacterInfo,
  pub character_name: String,
  pub guild_name: String,
  pub guild_rank: String
}