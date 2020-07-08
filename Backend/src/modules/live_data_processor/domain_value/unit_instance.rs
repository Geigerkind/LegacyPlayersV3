#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UnitInstance {
    pub entered: u64,
    pub map_id: u16,
    pub map_difficulty: u8,
    pub instance_id: u32,
}
