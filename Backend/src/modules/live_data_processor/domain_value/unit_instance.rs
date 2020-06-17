#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UnitInstance {
    pub map_id: u32,
    pub map_difficulty: u8,
    pub instance_id: u32,
}
