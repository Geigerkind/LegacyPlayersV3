#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpeedRun {
    pub instance_meta_id: u32,
    pub map_id: u16,
    pub guild_id: u32,
    pub guild_name: String,
    pub server_id: u32,
    pub duration: u64,
}