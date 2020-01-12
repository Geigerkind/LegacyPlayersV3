use crate::modules::armory::dto::CharacterGearDto;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CharacterInfoDto {
  pub gear: CharacterGearDto,
  pub hero_class_id: u8,
  pub level: u8,
  pub gender: bool,
  pub profession1: Option<u8>,
  pub profession2: Option<u8>,
  pub talent_specialization: Option<String>,
  pub faction: bool,
  pub race_id: u8,
}