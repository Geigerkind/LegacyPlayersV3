use crate::modules::armory::{
    domain_value::CharacterItem,
    dto::{CharacterGearDto, CharacterItemDto},
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterGear {
    pub id: u32,
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

impl PartialEq for CharacterGear {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl CharacterGear {
    pub fn deep_eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.head.is_eq(&other.head)
            && self.neck.is_eq(&other.neck)
            && self.shoulder.is_eq(&other.shoulder)
            && self.back.is_eq(&other.back)
            && self.chest.is_eq(&other.chest)
            && self.shirt.is_eq(&other.shirt)
            && self.tabard.is_eq(&other.tabard)
            && self.wrist.is_eq(&other.wrist)
            && self.main_hand.is_eq(&other.main_hand)
            && self.off_hand.is_eq(&other.off_hand)
            && self.ternary_hand.is_eq(&other.ternary_hand)
            && self.glove.is_eq(&other.glove)
            && self.belt.is_eq(&other.belt)
            && self.leg.is_eq(&other.leg)
            && self.boot.is_eq(&other.boot)
            && self.ring1.is_eq(&other.ring1)
            && self.ring2.is_eq(&other.ring2)
            && self.trinket1.is_eq(&other.trinket1)
            && self.trinket2.is_eq(&other.trinket2)
    }

    pub fn compare_by_value(&self, other: &CharacterGearDto) -> bool {
        self.head.is_eq_by_value(&other.head)
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
            && self.trinket2.is_eq_by_value(&other.trinket2)
    }
}

trait EqByValue {
    fn is_eq_by_value(&self, other: &Option<CharacterItemDto>) -> bool;
    fn is_eq(&self, other: &Option<CharacterItem>) -> bool;
}

impl EqByValue for Option<CharacterItem> {
    fn is_eq_by_value(&self, other: &Option<CharacterItemDto>) -> bool {
        (self.is_some() && other.is_some() && self.as_ref().unwrap().compare_by_value(other.as_ref().unwrap())) || (self.is_none() && other.is_none())
    }

    fn is_eq(&self, other: &Option<CharacterItem>) -> bool {
        (self.is_none() && other.is_none()) || (self.is_some() && other.is_some() && self.as_ref().unwrap().deep_eq(other.as_ref().unwrap()))
    }
}
