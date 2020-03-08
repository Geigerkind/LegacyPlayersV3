use crate::modules::armory::dto::GuildDto;
use crate::dto::CheckPlausability;
use crate::modules::armory::domain_value::GuildRank;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterGuildDto {
  pub guild: GuildDto,
  pub rank: GuildRank
}

impl CheckPlausability for CharacterGuildDto {
  fn is_plausible(&self) -> bool {
    self.guild.is_plausible()
        && self.rank.is_plausible()
  }
}