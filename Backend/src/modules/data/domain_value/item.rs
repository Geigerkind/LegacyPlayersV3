#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Item {
    pub expansion_id: u8,
    pub id: u32,
    pub localization_id: u32,
    pub icon: u16,
    pub quality: u8,
    pub inventory_type: Option<u8>,
    pub class_id: u8,
    pub required_level: Option<u8>,
    pub bonding: Option<u8>,
    pub sheath: Option<u8>,
    pub itemset: Option<u16>,
    pub max_durability: Option<u16>,
    pub item_level: Option<u16>,
    pub delay: Option<u16>,
}
