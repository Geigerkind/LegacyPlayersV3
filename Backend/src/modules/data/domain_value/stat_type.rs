#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct StatType {
    pub id: u8,
    pub localization_id: u32,
}
