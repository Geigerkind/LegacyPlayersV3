use crate::dto::TableFilter;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterSearchFilter {
  pub page: u32,
  pub gender: TableFilter<u8>,
  pub race: TableFilter<u8>,
  pub faction: TableFilter<u8>,
  pub server: TableFilter<u32>,
  pub hero_class: TableFilter<u8>,
  pub name: TableFilter<String>,
  pub guild: TableFilter<String>,
  pub last_updated: TableFilter<u64>,
}