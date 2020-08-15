use crate::modules::armory::{domain_value::GuildRank, dto::GuildViewerMemberDto};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GuildViewerDto {
    pub guild_id: u32,
    pub guild_name: String,
    pub ranks: Vec<GuildRank>,
    pub member: Vec<GuildViewerMemberDto>,
}
