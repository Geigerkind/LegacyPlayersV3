use mysql_connection::tools::Execute;

use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::CharacterInfo;
use crate::modules::armory::dto::{ArmoryFailure, CharacterInfoDto};
use crate::modules::armory::tools::{CreateCharacterGear, GetCharacterInfo};

pub trait CreateCharacterInfo {
  fn create_character_info(&self, character_info: CharacterInfoDto) -> Result<CharacterInfo, ArmoryFailure>;
}

impl CreateCharacterInfo for Armory {
  fn create_character_info(&self, character_info: CharacterInfoDto) -> Result<CharacterInfo, ArmoryFailure> {
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

    // Check if this breakdown is effectively null, i.e. 000|000|000
    let mut talent_specialization = None;
    if character_info.talent_specialization.is_some() {
      if character_info.talent_specialization.as_ref().unwrap()
          .split('|').map(|spec| spec.chars().map(|talent| talent.to_digit(10)
          .unwrap()).sum::<u32>()).sum::<u32>() > 0 {
        talent_specialization = character_info.talent_specialization.clone();
      }
    }

    let params = params!(
      "gear_id" => gear.id,
      "hero_class_id" => character_info.hero_class_id,
      "level" => character_info.level,
      "gender" => character_info.gender,
      "profession1" => character_info.profession1.clone(),
      "profession2" => character_info.profession2.clone(),
      "talent_specialization" => talent_specialization,
      "race_id" => character_info.race_id
    );

    // It may fail due to the unique constraint if a race condition occurs
    self.db_main.execute_wparams("INSERT INTO armory_character_info (`gear_id`, `hero_class_id`, `level`, `gender`, `profession1`, `profession2`, `talent_specialization`, `race_id`) VALUES (:gear_id, :hero_class_id, :level, :gender, :profession1, :profession2, :talent_specialization, :race_id)", params.clone());
    let char_info = self.get_character_info_by_value(character_info.to_owned());
    if char_info.is_ok() {
      return Ok(char_info.unwrap());
    }
    Err(ArmoryFailure::Database("create_character_info".to_owned()))
  }
}