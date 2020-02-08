use crate::modules::transport_layer::GuildDto;

#[derive(Debug, Clone)]
pub struct CharacterGuildDto {
  pub guild: GuildDto,
  pub rank: String
}