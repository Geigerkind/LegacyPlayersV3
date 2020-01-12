use crate::dto::Failure;
use crate::modules::armory::domain_value::Gear;
use crate::modules::armory::Armory;
use mysql_connection::tools::Select;
use crate::modules::armory::tools::GetCharacterItem;

pub trait GetGear {
  fn get_gear(&self, gear_id: u32) -> Result<Gear, Failure>;
  // Note: Gear = GearDto here
  fn get_gear_by_value(&self, gear: Gear) -> Result<Gear, Failure>;
}

impl GetGear for Armory {
  fn get_gear(&self, gear_id: u32) -> Result<Gear, Failure> {
    let params = params!(
      "id" => gear_id
    );
    // Note: This implementation should not be very fast
    self.db_main.select_wparams_value("SELECT * FROM armory_gear WHERE id=:id", &|mut row| {
      Gear {
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
      }
    }, params);
    Err(Failure::Unknown)
  }

  fn get_gear_by_value(&self, gear: Gear) -> Result<Gear, Failure> {
    unimplemented!()
  }
}