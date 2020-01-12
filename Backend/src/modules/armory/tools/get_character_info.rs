use mysql_connection::tools::Select;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::CharacterInfo;
use crate::modules::armory::dto::CharacterInfoDto;
use crate::modules::armory::tools::GetGear;

pub trait GetCharacterInfo {
  fn get_character_info(&self, character_info_id: u32) -> Result<CharacterInfo, Failure>;
  fn get_character_info_by_value(&self, character_info: CharacterInfoDto) -> Result<CharacterInfo, Failure>;
}

impl GetCharacterInfo for Armory {
  fn get_character_info(&self, character_info_id: u32) -> Result<CharacterInfo, Failure> {
    let params = params!(
      "id" => character_info_id
    );
    self.db_main.select_wparams_value("SELECT * FROM armory_character_info WHERE id=:id", &|mut row| {
      CharacterInfo {
        id: row.take(0).unwrap(),
        gear: self.get_gear(row.take(1).unwrap()).unwrap(),
        hero_class_id: row.take(2).unwrap(),
        level: row.take(3).unwrap(),
        gender: row.take(4).unwrap(),
        profession1: row.take_opt(5).unwrap().ok(),
        profession2: row.take_opt(6).unwrap().ok(),
        talent_specialization: row.take_opt(7).unwrap().ok(),
        faction: row.take(8).unwrap(),
        race_id: row.take(9).unwrap()
      }
    }, params);
    Err(Failure::Unknown)
  }

  fn get_character_info_by_value(&self, character_info: CharacterInfoDto) -> Result<CharacterInfo, Failure> {
    let params = params!(
      "gear_id" => self.get_gear_by_value(character_info.gear.clone()).unwrap().id,
      "hero_class" => character_info.hero_class_id,
      "level" => character_info.level,
      "gender" => character_info.gender,
      "profession1" => character_info.profession1.clone(),
      "profession2" => character_info.profession2.clone(),
      "talent_specialization" => character_info.talent_specialization.clone(),
      "faction" => character_info.faction,
      "race" => character_info.race_id
    );
    self.db_main.select_wparams_value("SELECT id FROM armory_character_info WHERE gear_id=:gear_id AND hero_class=:hero_class AND level=:level AND gender=:gender AND profession1=:profession1 AND profession2=:profession2 AND talent_specialization=:talent_specialization AND faction=:faction AND race=:race", &|mut row| {
      return self.get_character_info(row.take(0).unwrap());
    }, params);
    Err(Failure::Unknown)
  }
}