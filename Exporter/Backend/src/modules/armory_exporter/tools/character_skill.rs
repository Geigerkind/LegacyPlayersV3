use crate::modules::armory_exporter::domain_value::CharacterSkillTable;
use crate::modules::ArmoryExporter;
use mysql_connection::tools::Select;

pub trait RetrieveCharacterSkills {
  fn get_profession_skills(&self, character_id: u32) -> Vec<CharacterSkillTable>;
}

impl RetrieveCharacterSkills for ArmoryExporter {
  fn get_profession_skills(&self, character_id: u32) -> Vec<CharacterSkillTable> {
    self.db_characters.select_wparams("SELECT skill, value, max FROM character_skills \
      WHERE guid=:character_id AND skill IN (164,165,171,182,186,197,202,333,393,755,773)", &|mut row| CharacterSkillTable {
      character_id,
      skill_id: row.take(0).unwrap(),
      value: row.take(1).unwrap(),
      max: row.take(2).unwrap()
    }, params!(
      "character_id" => character_id
    )).to_vec()
  }
}