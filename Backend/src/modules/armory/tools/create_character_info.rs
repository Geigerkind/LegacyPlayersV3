use crate::dto::Failure;
use crate::modules::armory::domain_value::CharacterInfo;
use crate::modules::armory::Armory;
use mysql_connection::tools::Execute;
use crate::modules::armory::tools::{CreateGear, GetCharacterInfo};

pub trait CreateCharacterInfo {
  // Here CharacterInfo = CharacterInfoDto
  fn create_character_info(&self, character_info: CharacterInfo) -> Result<CharacterInfo, Failure>;
}

impl CreateCharacterInfo for Armory {
  fn create_character_info(&self, character_info: CharacterInfo) -> Result<CharacterInfo, Failure> {
    let gear_res = self.create_gear(character_info.gear.to_owned());
    if gear_res.is_err() {
      return Err(gear_res.err().unwrap());
    }
    let gear = gear_res.unwrap();

    let params = params!(
      "gear_id" => gear.id,
      "hero_class" => character_info.hero_class_id,
      "level" => character_info.level,
      "gender" => character_info.gender,
      "profession1" => character_info.profession1.clone(),
      "profession2" => character_info.profession2.clone(),
      "talent_specialization" => character_info.talent_specialization.clone(),
      "faction" => character_info.faction,
      "race" => character_info.race_id
    );
    if self.db_main.execute_wparams("INSERT INTO armory_character_info (`gear_id`, `hero_class`, `level`, `gender`, `profession1`, `profession2`, `talent_specialization`, `faction`, `race`) VALUES (:gear_id, :hero_class, :level, :gender, :profession1, :profession2, :talent_specialization, :faction, :race)", params.clone()) {
      return self.get_character_info_by_value(character_info.to_owned());
    }

    Err(Failure::Unknown)
  }
}