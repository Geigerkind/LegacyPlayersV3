#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Gem {
    pub expansion_id: u8,
    pub item_id: u32,
    pub enchant_id: u32,
    pub flag: u8,
}
