use crate::modules::armory::material::{Guild, Character};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterSearchResult {
  pub character: Character,
  pub guild: Option<Guild>,
  pub faction: bool
}