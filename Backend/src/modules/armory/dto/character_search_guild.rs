#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterSearchGuildDto {
  pub guild_id: u32,
  pub name: String
}