#[derive(Debug, Serialize)]
pub struct CharacterWithConsent {
  pub character_id: u32,
  pub character_name: String,
  pub consent: bool
}