use crate::modules::armory::dto::GuildDto;
use crate::dto::CheckPlausability;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterGuildDto {
  pub guild: GuildDto,
  pub rank: String
}

impl CheckPlausability for CharacterGuildDto {
  fn is_plausible(&self) -> bool {
    self.guild.is_plausible()
      && !self.rank.is_empty()
  }
}