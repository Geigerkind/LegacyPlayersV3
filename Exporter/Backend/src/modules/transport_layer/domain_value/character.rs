use crate::modules::transport_layer::CharacterHistoryDto;

#[derive(Debug, Clone)]
pub struct CharacterDto {
  pub server_uid: u64,
  pub character_history: Option<CharacterHistoryDto>,
}