use crate::modules::armory::material::Gear;

#[derive(Debug)]
pub struct CharacterInfo {
  pub id: u32,
  pub gear: Gear,
  pub hero_class_id: u8,
  pub level: u8,
  pub gender: bool,
  pub profession1: Option<u8>,
  pub profession2: Option<u8>,
  pub talent_specification: Option<String>,
  pub faction: bool,
  pub race_id: u8
}