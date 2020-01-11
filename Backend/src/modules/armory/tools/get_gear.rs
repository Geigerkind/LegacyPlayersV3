use crate::dto::Failure;
use crate::modules::armory::domain_value::Gear;
use crate::modules::armory::Armory;

pub trait GetGear {
  fn get_gear(&self, gear_id: u32) -> Result<Gear, Failure>;
}

impl GetGear for Armory {
  fn get_gear(&self, gear_id: u32) -> Result<Gear, Failure> {
    unimplemented!()
  }
}