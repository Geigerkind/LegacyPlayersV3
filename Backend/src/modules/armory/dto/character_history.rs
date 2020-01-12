use crate::modules::armory::dto::{CharacterInfoDto, GuildDto};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterHistoryDto {
  pub character_uid: u64,
  pub character_info: CharacterInfoDto,
  pub character_name: String,
  pub guild: Option<GuildDto>,
  pub guild_rank: Option<String>
}