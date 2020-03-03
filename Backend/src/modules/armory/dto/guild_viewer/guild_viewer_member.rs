#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GuildViewerMemberDto {
    pub character_id: u32,
    pub character_name: String,
    pub faction: bool,
    pub race_id: u8,
    pub hero_class_id: u8,
    pub rank: String,
    pub last_seen: u64
}