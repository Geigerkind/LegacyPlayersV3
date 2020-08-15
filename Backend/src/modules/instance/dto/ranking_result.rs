#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RankingResult {
    pub attempt_id: u32,
    pub amount: u32,
    pub duration: u64,
}
