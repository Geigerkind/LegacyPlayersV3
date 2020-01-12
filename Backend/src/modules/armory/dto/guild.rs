#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GuildDto {
  pub server_uid: u64,
  pub name: String
}