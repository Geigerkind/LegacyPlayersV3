use mysql_connection::tools::Select;

use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::CharacterGear;
use crate::modules::armory::dto::{CharacterGearDto, ArmoryFailure};
use crate::modules::armory::tools::GetCharacterItem;

pub trait GetCharacterGear {
  fn get_character_gear(&self, gear_id: u32) -> Result<CharacterGear, ArmoryFailure>;
  fn get_character_gear_by_value(&self, gear: CharacterGearDto) -> Result<CharacterGear, ArmoryFailure>;
}

impl GetCharacterGear for Armory {
  fn get_character_gear(&self, gear_id: u32) -> Result<CharacterGear, ArmoryFailure> {
    let params = params!(
      "id" => gear_id
    );
    // Note: This implementation should not be very fast
    self.db_main.select_wparams_value("SELECT * FROM armory_gear WHERE id=:id", &|mut row| {
      Ok(CharacterGear {
        id: row.take(0).unwrap(),
        head: row.take_opt(1).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        neck: row.take_opt(2).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        shoulder: row.take_opt(3).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        back: row.take_opt(4).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        chest: row.take_opt(5).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        shirt: row.take_opt(6).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        tabard: row.take_opt(7).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        wrist: row.take_opt(8).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        main_hand: row.take_opt(9).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        off_hand: row.take_opt(10).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        ternary_hand: row.take_opt(11).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        glove: row.take_opt(12).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        belt: row.take_opt(13).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        leg: row.take_opt(14).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        boot: row.take_opt(15).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        ring1: row.take_opt(16).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        ring2: row.take_opt(17).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        trinket1: row.take_opt(18).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
        trinket2: row.take_opt(19).unwrap().ok().and_then(|id| self.get_character_item(id).ok()),
      })
    }, params).unwrap_or_else(|| Err(ArmoryFailure::Database("get_character_gear".to_owned())))
  }

  fn get_character_gear_by_value(&self, gear: CharacterGearDto) -> Result<CharacterGear, ArmoryFailure> {
    let head = gear.head.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let neck = gear.neck.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let shoulder = gear.shoulder.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let back = gear.back.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let chest = gear.chest.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let shirt = gear.shirt.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let tabard = gear.tabard.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let wrist = gear.wrist.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let main_hand = gear.main_hand.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let off_hand = gear.off_hand.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let ternary_hand = gear.ternary_hand.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let glove = gear.glove.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let belt = gear.belt.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let leg = gear.leg.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let boot = gear.boot.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let ring1 = gear.ring1.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let ring2 = gear.ring2.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let trinket1 = gear.trinket1.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());
    let trinket2 = gear.trinket2.and_then(|item| self.get_character_item_by_value(item.to_owned()).ok());

    let params = params!(
      "head" => head.as_ref().and_then(|item| Some(item.id)),
      "neck" => neck.as_ref().and_then(|item| Some(item.id)),
      "shoulder" => shoulder.as_ref().and_then(|item| Some(item.id)),
      "back" => back.as_ref().and_then(|item| Some(item.id)),
      "chest" => chest.as_ref().and_then(|item| Some(item.id)),
      "shirt" => shirt.as_ref().and_then(|item| Some(item.id)),
      "tabard" => tabard.as_ref().and_then(|item| Some(item.id)),
      "wrist" => wrist.as_ref().and_then(|item| Some(item.id)),
      "main_hand" => main_hand.as_ref().and_then(|item| Some(item.id)),
      "off_hand" => off_hand.as_ref().and_then(|item| Some(item.id)),
      "ternary_hand" => ternary_hand.as_ref().and_then(|item| Some(item.id)),
      "glove" => glove.as_ref().and_then(|item| Some(item.id)),
      "belt" => belt.as_ref().and_then(|item| Some(item.id)),
      "leg" => leg.as_ref().and_then(|item| Some(item.id)),
      "boot" => boot.as_ref().and_then(|item| Some(item.id)),
      "ring1" => ring1.as_ref().and_then(|item| Some(item.id)),
      "ring2" => ring2.as_ref().and_then(|item| Some(item.id)),
      "trinket1" => trinket1.as_ref().and_then(|item| Some(item.id)),
      "trinket2" => trinket2.as_ref().and_then(|item| Some(item.id)),
    );
    // Note: This implementation should not be very fast
    self.db_main.select_wparams_value("SELECT id FROM armory_gear WHERE \
      ((ISNULL(:head) AND ISNULL(head)) OR head = :head) \
      AND ((ISNULL(:neck) AND ISNULL(neck)) OR neck = :neck) \
      AND ((ISNULL(:shoulder) AND ISNULL(shoulder)) OR shoulder = :shoulder) \
      AND ((ISNULL(:back) AND ISNULL(back)) OR back = :back) \
      AND ((ISNULL(:chest) AND ISNULL(chest)) OR chest = :chest) \
      AND ((ISNULL(:shirt) AND ISNULL(shirt)) OR shirt = :shirt) \
      AND ((ISNULL(:tabard) AND ISNULL(tabard)) OR tabard = :tabard) \
      AND ((ISNULL(:wrist) AND ISNULL(wrist)) OR wrist = :wrist) \
      AND ((ISNULL(:main_hand) AND ISNULL(main_hand)) OR main_hand = :main_hand) \
      AND ((ISNULL(:off_hand) AND ISNULL(off_hand)) OR off_hand = :off_hand) \
      AND ((ISNULL(:ternary_hand) AND ISNULL(ternary_hand)) OR ternary_hand = :ternary_hand) \
      AND ((ISNULL(:glove) AND ISNULL(glove)) OR glove = :glove) \
      AND ((ISNULL(:belt) AND ISNULL(belt)) OR belt = :belt) \
      AND ((ISNULL(:leg) AND ISNULL(leg)) OR leg = :leg) \
      AND ((ISNULL(:boot) AND ISNULL(boot)) OR boot = :boot) \
      AND ((ISNULL(:ring1) AND ISNULL(ring1)) OR ring1 = :ring1) \
      AND ((ISNULL(:ring2) AND ISNULL(ring2)) OR ring2 = :ring2) \
      AND ((ISNULL(:trinket1) AND ISNULL(trinket1)) OR trinket1 = :trinket1) \
      AND ((ISNULL(:trinket2) AND ISNULL(trinket2)) OR trinket2 = :trinket2)", &|mut row| {
      Ok(CharacterGear {
        id: row.take(0).unwrap(),
        head: head.to_owned(),
        neck: neck.to_owned(),
        shoulder: shoulder.to_owned(),
        back: back.to_owned(),
        chest: chest.to_owned(),
        shirt: shirt.to_owned(),
        tabard: tabard.to_owned(),
        wrist: wrist.to_owned(),
        main_hand: main_hand.to_owned(),
        off_hand: off_hand.to_owned(),
        ternary_hand: ternary_hand.to_owned(),
        glove: glove.to_owned(),
        belt: belt.to_owned(),
        leg: leg.to_owned(),
        boot: boot.to_owned(),
        ring1: ring1.to_owned(),
        ring2: ring2.to_owned(),
        trinket1: trinket1.to_owned(),
        trinket2: trinket2.to_owned(),
      })
    }, params).unwrap_or_else(|| Err(ArmoryFailure::Database("get_character_gear_by_value".to_owned())))
  }
}