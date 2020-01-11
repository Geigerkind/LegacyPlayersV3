use crate::modules::armory::dto::CharacterHistoryDto;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterDto {
  pub server_uid: u64,
  pub character_history: Option<CharacterHistoryDto>,
}