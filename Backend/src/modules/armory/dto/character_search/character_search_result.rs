use crate::modules::armory::dto::{CharacterSearchGuildDto, CharacterSearchCharacterDto};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterSearchResult {
  pub guild: Option<CharacterSearchGuildDto>,
  pub character: CharacterSearchCharacterDto,
  pub faction: bool,
  pub timestamp: u64
}