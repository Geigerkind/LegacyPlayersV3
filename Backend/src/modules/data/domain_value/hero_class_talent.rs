#[derive(Debug, Clone, Serialize, JsonSchema, Copy)]
pub struct HeroClassTalent {
  pub icon: u16,
  pub localization_id: u32
}