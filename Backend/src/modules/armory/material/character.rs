use crate::modules::armory::material::CharacterHistory;

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Character {
  pub id: u32,
  pub server_id: u32,
  pub server_uid: u64,
  pub last_update: Option<CharacterHistory>,
  pub history_ids: Vec<u32>,
}