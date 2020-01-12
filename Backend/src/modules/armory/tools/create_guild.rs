use mysql_connection::tools::Execute;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::Guild;
use crate::modules::armory::tools::GetGuild;

pub trait CreateGuild {
  fn create_guild(&self, server_id: u32, name: String) -> Result<Guild, Failure>;
}

impl CreateGuild for Armory {
  fn create_guild(&self, server_id: u32, name: String) -> Result<Guild, Failure> {
    // Check if it already exists, if so return existing one
    let existing_guild = self.get_guild_by_name(server_id, name.clone());
    if existing_guild.is_some() {
      return Ok(existing_guild.unwrap());
    }

    // Else create one
    let mut guilds = self.guilds.write().unwrap();
    if self.db_main.execute_wparams("INSERT INTO armory_guild (`server_id`, `guild_name`) VALUES (:server_id, :guild_name)", params!(
      "server_id" => server_id,
      "guild_name" => name.to_owned()
    )) {
      let new_guild = self.get_guild_by_name(server_id, name.to_owned()).unwrap();
      guilds.insert(new_guild.id, new_guild.clone());
      return Ok(new_guild);
    }

    Err(Failure::Unknown)
  }
}