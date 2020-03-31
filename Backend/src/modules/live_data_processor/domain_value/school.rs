#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum School {
  Physical,
  Holy,
  Fire,
  Nature,
  Frost,
  Shadow,
  Arcane
}