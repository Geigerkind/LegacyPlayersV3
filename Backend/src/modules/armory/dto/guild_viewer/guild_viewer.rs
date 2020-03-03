use crate::modules::armory::dto::GuildViewerMemberDto;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GuildViewerDto {
    pub guild_id: u32,
    pub guild_name: String,
    pub member: Vec<GuildViewerMemberDto>
}