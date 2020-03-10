use crate::dto::CheckPlausability;
use crate::modules::armory::dto::CharacterItemDto;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterGearDto {
    pub head: Option<CharacterItemDto>,
    pub neck: Option<CharacterItemDto>,
    pub shoulder: Option<CharacterItemDto>,
    pub back: Option<CharacterItemDto>,
    pub chest: Option<CharacterItemDto>,
    pub shirt: Option<CharacterItemDto>,
    pub tabard: Option<CharacterItemDto>,
    pub wrist: Option<CharacterItemDto>,
    pub main_hand: Option<CharacterItemDto>,
    pub off_hand: Option<CharacterItemDto>,
    pub ternary_hand: Option<CharacterItemDto>,
    pub glove: Option<CharacterItemDto>,
    pub belt: Option<CharacterItemDto>,
    pub leg: Option<CharacterItemDto>,
    pub boot: Option<CharacterItemDto>,
    pub ring1: Option<CharacterItemDto>,
    pub ring2: Option<CharacterItemDto>,
    pub trinket1: Option<CharacterItemDto>,
    pub trinket2: Option<CharacterItemDto>,
}

impl CheckPlausability for CharacterGearDto {
    fn is_plausible(&self) -> bool {
        self.head.is_plausible()
            && self.neck.is_plausible()
            && self.shoulder.is_plausible()
            && self.back.is_plausible()
            && self.chest.is_plausible()
            && self.shirt.is_plausible()
            && self.tabard.is_plausible()
            && self.wrist.is_plausible()
            && self.main_hand.is_plausible()
            && self.off_hand.is_plausible()
            && self.ternary_hand.is_plausible()
            && self.glove.is_plausible()
            && self.belt.is_plausible()
            && self.leg.is_plausible()
            && self.boot.is_plausible()
            && self.ring1.is_plausible()
            && self.ring2.is_plausible()
            && self.trinket1.is_plausible()
            && self.trinket2.is_plausible()
    }
}

impl CheckPlausability for Option<CharacterItemDto> {
    fn is_plausible(&self) -> bool {
        self.is_none() || self.as_ref().unwrap().is_plausible()
    }
}
