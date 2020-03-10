#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Expansion {
    pub id: u8,
    pub localization_id: u32,
}
