use crate::modules::armory::domain_value::CharacterItem;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Gear {
  pub head: Option<CharacterItem>,
  pub neck: Option<CharacterItem>,
  pub shoulder: Option<CharacterItem>,
  pub back: Option<CharacterItem>,
  pub chest: Option<CharacterItem>,
  pub shirt: Option<CharacterItem>,
  pub tabard: Option<CharacterItem>,
  pub wrist: Option<CharacterItem>,
  pub main_hand: Option<CharacterItem>,
  pub off_hand: Option<CharacterItem>,
  pub ternary_hand: Option<CharacterItem>,
  pub glove: Option<CharacterItem>,
  pub belt: Option<CharacterItem>,
  pub leg: Option<CharacterItem>,
  pub boot: Option<CharacterItem>,
  pub ring1: Option<CharacterItem>,
  pub ring2: Option<CharacterItem>,
  pub trinket1: Option<CharacterItem>,
  pub trinket2: Option<CharacterItem>,
}