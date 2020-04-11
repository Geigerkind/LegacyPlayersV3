use crate::modules::armory::material::Character;

// TODO: This may not use materials (?)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Player {
  pub character: Character
}

impl PartialEq for Player {
  fn eq(&self, other: &Self) -> bool {
    self.character.id == other.character.id
  }
}