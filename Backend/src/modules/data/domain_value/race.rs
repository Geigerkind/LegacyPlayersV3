#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Race {
    pub id: u8,
    pub localization_id: u32,
    pub faction: bool,
}
