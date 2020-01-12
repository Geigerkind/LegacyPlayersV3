use crate::dto::Failure;
use crate::modules::armory::domain_value::Guild;
use crate::modules::armory::Armory;

pub trait CreateGuild {
  fn create_guild(&self, server_id: u32, name: String) -> Result<Guild, Failure>;
}

impl CreateGuild for Armory {
  fn create_guild(&self, server_id: u32, name: String) -> Result<Guild, Failure> {
    unimplemented!()
  }
}