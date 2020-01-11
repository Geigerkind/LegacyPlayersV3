use crate::modules::armory::dto::CreateCharacterHistory;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateCharacter {
  pub server_uid: u64,
  pub character_history: Option<CreateCharacterHistory>,
}