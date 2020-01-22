use crate::modules::armory::dto::GuildDto;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterGuildDto {
  pub guild: GuildDto,
  pub rank: String
}