use crate::dto::SelectOption;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterSearchResult {
  pub gender: SelectOption<u8>,
  pub race: SelectOption<u8>,
  pub faction: SelectOption<u8>,
  pub server: SelectOption<u32>,
  pub hero_class: SelectOption<u8>,
  pub name: String,
  pub last_updated: u64
}