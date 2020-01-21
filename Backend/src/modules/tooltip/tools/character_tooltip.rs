use crate::modules::tooltip::material::CharacterTooltip;
use crate::dto::Failure;
use crate::modules::tooltip::Tooltip;
use crate::modules::armory::Armory;

pub trait RetrieveCharacterTooltip {
    fn get_character(&self, armory: &Armory, character_id: u32) -> Result<CharacterTooltip, Failure>;
}

impl RetrieveCharacterTooltip for Tooltip {
    fn get_character(&self, armory: &Armory, character_id: u32) -> Result<CharacterTooltip, Failure> {
        unimplemented!()
    }
}