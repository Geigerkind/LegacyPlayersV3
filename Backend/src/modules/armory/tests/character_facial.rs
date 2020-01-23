use crate::modules::armory::Armory;
use crate::modules::armory::dto::CharacterFacialDto;
use crate::modules::armory::tools::{CreateCharacterFacial, GetCharacterFacial};
use mysql_connection::tools::Execute;

#[test]
fn character_facial() {
  let armory = Armory::default();
  let character_facial_dto = CharacterFacialDto {
    skin_color: 1,
    face_style: 2,
    hair_style: 1,
    hair_color: 2,
    facial_hair: 1
  };

  let character_facial_res = armory.create_character_facial(character_facial_dto.clone());
  assert!(character_facial_res.is_ok());

  let character_facial = character_facial_res.unwrap();
  assert!(character_facial.compare_by_value(&character_facial_dto));

  let character_facial2_res = armory.get_character_facial(character_facial.id);
  assert!(character_facial2_res.is_ok());

  let character_facial2 = character_facial2_res.unwrap();
  assert!(character_facial2.deep_eq(&character_facial));

  armory.db_main.execute_wparams("DELETE FROM armory_character_facial WHERE id=:id", params!("id" => character_facial.id));
}