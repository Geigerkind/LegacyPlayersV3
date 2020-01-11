use crate::dto::Failure;
use crate::modules::armory::domain_value::Gear;
use crate::modules::armory::Armory;

pub trait CreateGear {
  // Note: Gear = GearDto here
  fn create_gear(&self, gear: Gear) -> Result<Gear, Failure>;
}

impl CreateGear for Armory {
  fn create_gear(&self, gear: Gear) -> Result<Gear, Failure> {
    unimplemented!()
  }
}