#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UnitInstance {
    pub instance_meta_id: u32,
    pub entered: u64,
    pub map_id: u16,
    pub map_difficulty: u8,
    pub instance_id: u32,
}
