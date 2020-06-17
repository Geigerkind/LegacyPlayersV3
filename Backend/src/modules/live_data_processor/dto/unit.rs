#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Unit {
    pub is_player: bool,
    pub unit_id: u64,
}
