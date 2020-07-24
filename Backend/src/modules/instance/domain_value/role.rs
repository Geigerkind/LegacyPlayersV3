#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Role {
    Tank,
    Healer,
    Dps,
}
