use crate::modules::armory::domain_value::CharacterFacial;
use crate::modules::armory::dto::{ArmoryFailure, CharacterFacialDto};
use crate::modules::armory::tools::GetCharacterFacial;
use crate::modules::armory::Armory;
use mysql_connection::tools::Execute;

pub trait CreateCharacterFacial {
    fn create_character_facial(
        &self,
        character_facial_dto: CharacterFacialDto,
    ) -> Result<CharacterFacial, ArmoryFailure>;
}

impl CreateCharacterFacial for Armory {
    fn create_character_facial(
        &self,
        character_facial_dto: CharacterFacialDto,
    ) -> Result<CharacterFacial, ArmoryFailure> {
        // If it already exists, return this one
        let existing_facial = self.get_character_facial_by_value(character_facial_dto.clone());
        if existing_facial.is_ok() {
            return existing_facial;
        }

        let params = params!(
          "skin_color" => character_facial_dto.skin_color,
          "face_style" => character_facial_dto.face_style,
          "hair_style" => character_facial_dto.hair_style,
          "hair_color" => character_facial_dto.hair_color,
          "facial_hair" => character_facial_dto.facial_hair,
        );

        // It may fail due to the unique constraint if a race condition occurs
        self.db_main.execute_wparams("INSERT INTO armory_character_facial (`skin_color`, `face_style`, `hair_style`, `hair_color`, `facial_hair`) VALUES (:skin_color, :face_style, :hair_style, :hair_color, :facial_hair)", params);
        if let Ok(char_facial) = self.get_character_facial_by_value(character_facial_dto) {
            return Ok(char_facial);
        }

        Err(ArmoryFailure::Database(
            "create_character_facial".to_owned(),
        ))
    }
}
