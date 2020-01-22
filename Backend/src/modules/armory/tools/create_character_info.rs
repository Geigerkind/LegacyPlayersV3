use mysql_connection::tools::Execute;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::CharacterInfo;
use crate::modules::armory::dto::CharacterInfoDto;
use crate::modules::armory::tools::{CreateCharacterGear, GetCharacterInfo};

pub trait CreateCharacterInfo {
  fn create_character_info(&self, character_info: CharacterInfoDto) -> Result<CharacterInfo, Failure>;
}

impl CreateCharacterInfo for Armory {
  fn create_character_info(&self, character_info: CharacterInfoDto) -> Result<CharacterInfo, Failure> {
    // Return existing one first
    let existing_character_info = self.get_character_info_by_value(character_info.clone());
    if existing_character_info.is_ok() {
      return existing_character_info;
    }

    // Create the gear needed
    let gear_res = self.create_character_gear(character_info.gear.to_owned());
    if gear_res.is_err() {
      return Err(gear_res.err().unwrap());
    }
    let gear = gear_res.unwrap();

    let params = params!(
      "gear_id" => gear.id,
      "hero_class_id" => character_info.hero_class_id,
      "level" => character_info.level,
      "gender" => character_info.gender,
      "profession1" => character_info.profession1.clone(),
      "profession2" => character_info.profession2.clone(),
      "talent_specialization" => character_info.talent_specialization.clone(),
      "race_id" => character_info.race_id
    );
    if self.db_main.execute_wparams("INSERT INTO armory_character_info (`gear_id`, `hero_class_id`, `level`, `gender`, `profession1`, `profession2`, `talent_specialization`, `race_id`) VALUES (:gear_id, :hero_class_id, :level, :gender, :profession1, :profession2, :talent_specialization, :race_id)", params.clone()) {
      return self.get_character_info_by_value(character_info.to_owned());
    }

    Err(Failure::Unknown)
  }
}