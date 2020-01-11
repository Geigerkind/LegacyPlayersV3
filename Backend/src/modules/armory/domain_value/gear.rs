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

impl Gear {
  pub fn compare_by_value(&self, other: &Gear) -> bool {
    return self.head.is_eq_by_value(&other.head)
      && self.neck.is_eq_by_value(&other.neck)
      && self.shoulder.is_eq_by_value(&other.shoulder)
      && self.back.is_eq_by_value(&other.back)
      && self.chest.is_eq_by_value(&other.chest)
      && self.shirt.is_eq_by_value(&other.shirt)
      && self.tabard.is_eq_by_value(&other.tabard)
      && self.wrist.is_eq_by_value(&other.wrist)
      && self.main_hand.is_eq_by_value(&other.main_hand)
      && self.off_hand.is_eq_by_value(&other.off_hand)
      && self.ternary_hand.is_eq_by_value(&other.ternary_hand)
      && self.glove.is_eq_by_value(&other.glove)
      && self.belt.is_eq_by_value(&other.belt)
      && self.leg.is_eq_by_value(&other.leg)
      && self.boot.is_eq_by_value(&other.boot)
      && self.ring1.is_eq_by_value(&other.ring1)
      && self.ring2.is_eq_by_value(&other.ring2)
      && self.trinket1.is_eq_by_value(&other.trinket1)
      && self.trinket2.is_eq_by_value(&other.trinket2);
  }
}

trait EqByValue {
  fn is_eq_by_value(&self, other: &Option<CharacterItem>) -> bool;
}

impl EqByValue for Option<CharacterItem> {
  fn is_eq_by_value(&self, other: &Option<CharacterItem>) -> bool {
    return (self.is_some() && other.is_some() && self.as_ref().unwrap().compare_by_value(other.as_ref().unwrap()))
      || (self.is_none() && other.is_none());
  }
}