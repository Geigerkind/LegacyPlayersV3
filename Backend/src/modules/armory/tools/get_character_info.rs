use crate::modules::armory::domain_value::CharacterInfo;
use crate::dto::Failure;
use crate::modules::armory::Armory;
use mysql_connection::tools::Select;

pub trait GetCharacterInfo {
  fn get_character_info(&self, character_info_id: u32) -> Result<CharacterInfo, Failure>;
  fn get_character_info_by_value(&self, character_info: CharacterInfo) -> Result<CharacterInfo, Failure>;
}

impl GetCharacterInfo for Armory {
  fn get_character_info(&self, character_info_id: u32) -> Result<CharacterInfo, Failure> {
    unimplemented!()
  }

  fn get_character_info_by_value(&self, character_info: CharacterInfo) -> Result<CharacterInfo, Failure> {
    let params = params!(
      "gear_id" => character_info.gear.id,
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
      let mut new_character_info = character_info.clone();
      new_character_info.id = row.take(1).unwrap();
      new_character_info
    }, params);
    Err(Failure::Unknown)
  }
}