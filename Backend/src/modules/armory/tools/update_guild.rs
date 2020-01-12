use mysql_connection::tools::Execute;

use crate::dto::Failure;
use crate::modules::armory::Armory;

pub trait UpdateGuild {
  fn update_guild_name(&self, id: u32, guild_name: String) -> Result<(), Failure>;
}

impl UpdateGuild for Armory {
  fn update_guild_name(&self, id: u32, guild_name: String) -> Result<(), Failure> {
    let mut guilds = self.guilds.write().unwrap();
    if self.db_main.execute_wparams("UPDATE armory_guild SET guild_name=:guild_name WHERE id=:id", params!(
      "id" => id,
      "guild_name" => guild_name.clone()
    )) {
      let mut guild = guilds.get_mut(&id).unwrap();
      guild.name = guild_name.to_owned();
      return Ok(());
    }
    Err(Failure::InvalidInput)
  }
}