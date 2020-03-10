#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Stat {
    pub stat_type: u8,
    pub stat_value: u16,
}
