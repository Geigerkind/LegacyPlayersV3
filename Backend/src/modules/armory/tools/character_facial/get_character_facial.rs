use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::CharacterFacial;
use mysql_connection::tools::Select;
use crate::modules::armory::dto::{CharacterFacialDto, ArmoryFailure};

pub trait GetCharacterFacial {
  fn get_character_facial(&self, facial_id: u32) -> Result<CharacterFacial, ArmoryFailure>;
  fn get_character_facial_by_value(&self, character_facial_dto: CharacterFacialDto) -> Result<CharacterFacial, ArmoryFailure>;
}

impl GetCharacterFacial for Armory {
  fn get_character_facial(&self, facial_id: u32) -> Result<CharacterFacial, ArmoryFailure> {
    let params = params!(
      "id" => facial_id
    );
    self.db_main.select_wparams_value("SELECT * FROM armory_character_facial WHERE id=:id", &|mut row| {
      Ok(CharacterFacial {
        id: row.take(0).unwrap(),
        skin_color: row.take(1).unwrap(),
        face_style: row.take(2).unwrap(),
        hair_style: row.take(3).unwrap(),
        hair_color: row.take(4).unwrap(),
        facial_hair: row.take(5).unwrap(),
      })
    }, params).unwrap_or_else(|| Err(ArmoryFailure::Database("get_character_facial".to_owned())))
  }

  fn get_character_facial_by_value(&self, character_facial_dto: CharacterFacialDto) -> Result<CharacterFacial, ArmoryFailure> {
    let params = params!(
      "skin_color" => character_facial_dto.skin_color,
      "face_style" => character_facial_dto.face_style,
      "hair_style" => character_facial_dto.hair_style,
      "hair_color" => character_facial_dto.hair_color,
      "facial_hair" => character_facial_dto.facial_hair,
    );
    self.db_main.select_wparams_value("SELECT * FROM armory_character_facial WHERE \
      skin_color=:skin_color \
      AND face_style=:face_style \
      AND hair_style=:hair_style \
      AND hair_color=:hair_color \
      AND facial_hair=:facial_hair", &|mut row| {
      Ok(CharacterFacial {
        id: row.take(0).unwrap(),
        skin_color: row.take(1).unwrap(),
        face_style: row.take(2).unwrap(),
        hair_style: row.take(3).unwrap(),
        hair_color: row.take(4).unwrap(),
        facial_hair: row.take(5).unwrap(),
      })
    }, params).unwrap_or_else(|| Err(ArmoryFailure::Database("get_character_facial_by_value".to_owned())))
  }
}