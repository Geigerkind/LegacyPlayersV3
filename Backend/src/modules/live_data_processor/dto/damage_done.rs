#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct DamageDone {
  pub attacker: u64,
  pub victim: u64,
  pub spell_id: Option<u32>,
  pub blocked: u32,
  pub school: u8,
  pub damage: u32,
  pub resisted_or_glanced: u32,
  pub absorbed: u32
}