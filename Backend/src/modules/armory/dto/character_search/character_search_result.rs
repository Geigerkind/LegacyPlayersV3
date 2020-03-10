use crate::modules::armory::dto::{CharacterSearchCharacterDto, CharacterSearchGuildDto};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterSearchResult {
    pub guild: Option<CharacterSearchGuildDto>,
    pub character: CharacterSearchCharacterDto,
    pub faction: bool,
    pub timestamp: u64,
}
