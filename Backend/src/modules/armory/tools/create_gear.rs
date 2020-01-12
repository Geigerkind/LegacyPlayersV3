use mysql_connection::tools::Execute;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::CharacterGear;
use crate::modules::armory::dto::CharacterGearDto;
use crate::modules::armory::tools::{CreateCharacterItem, GetGear};

pub trait CreateGear {
  fn create_gear(&self, character_gear: CharacterGearDto) -> Result<CharacterGear, Failure>;
}

impl CreateGear for Armory {
  fn create_gear(&self, character_gear: CharacterGearDto) -> Result<CharacterGear, Failure> {
    // Check if it already exists
    let existing_gear = self.get_gear_by_value(character_gear.clone());
    if existing_gear.is_ok() {
      return existing_gear;
    }

    // Note: This is extremely inefficient
    let params = params!(
      "head" => character_gear.head.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "neck" => character_gear.neck.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "shoulder" => character_gear.clone().shoulder.and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "back" => character_gear.back.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "chest" => character_gear.chest.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "shirt" => character_gear.shirt.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "tabard" => character_gear.tabard.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "wrist" => character_gear.wrist.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "main_hand" => character_gear.main_hand.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "off_hand" => character_gear.off_hand.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "ternary_hand" => character_gear.ternary_hand.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "glove" => character_gear.glove.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "belt" => character_gear.belt.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "leg" => character_gear.leg.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "boot" => character_gear.boot.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "ring1" => character_gear.ring1.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "ring2" => character_gear.ring2.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "trinket1" => character_gear.trinket1.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
      "trinket2" => character_gear.trinket2.clone().and_then(|item| self.create_character_item(item.to_owned()).ok().and_then(|created_item| Some(created_item.id))),
    );
    if self.db_main.execute_wparams("INSERT INTO armory_gear (`head`, `neck`, `shoulder`, `back`, `chest`, `tabard`, `wrist`, `main_hand`, `off_hand`, `ternary_hand`, `glove`, `belt`, `leg`, `boot`, `ring1`, `ring2`, `trinket1`, `trinket2`) VALUES (:head, :neck, :shoulder, :back, :chest, :shirt, :tabard, :wrist, :main_hand, :off_hand, :ternary_hand, :glove, :belt, :leg, :boot, :ring1, :ring2, :trinket1, :trinket2)", params.clone()) {
      return self.get_gear_by_value(character_gear);
    }

    Err(Failure::Unknown)
  }
}