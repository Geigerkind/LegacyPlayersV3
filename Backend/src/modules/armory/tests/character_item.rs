use crate::modules::armory::dto::CharacterItemDto;
use crate::modules::armory::Armory;
use crate::modules::armory::tools::{CreateCharacterItem, GetCharacterItem};
use mysql_connection::tools::Execute;

#[test]
fn create_character_item() {
  let armory = Armory::default().init();
  let character_item_dto = CharacterItemDto {
    item_id: 19019,
    random_property_id: None,
    enchant_id: Some(684),
    gem_ids: vec![None, None, None, None]
  };

  let character_item_res = armory.create_character_item(character_item_dto.clone());
  assert!(character_item_res.is_ok());

  let character_item = character_item_res.unwrap();
  assert_eq!(character_item.item_id, character_item_dto.item_id);
  assert_eq!(character_item.random_property_id, character_item_dto.random_property_id);
  assert_eq!(character_item.enchant_id, character_item_dto.enchant_id);
  assert_eq!(character_item.gem_ids, character_item_dto.gem_ids);

  let character_item2_res = armory.get_character_item(character_item.id);
  assert!(character_item2_res.is_ok());

  let character_item2 = character_item2_res.unwrap();
  assert_eq!(character_item2.id, character_item.id);
  assert_eq!(character_item2.item_id, character_item_dto.item_id);
  assert_eq!(character_item2.random_property_id, character_item_dto.random_property_id);
  assert_eq!(character_item2.enchant_id, character_item_dto.enchant_id);
  assert_eq!(character_item2.gem_ids, character_item_dto.gem_ids);

  armory.db_main.execute("DELETE FROM armory_item WHERE item_id = 19019");
}