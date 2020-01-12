use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::Guild;

pub trait GetGuild {
  fn get_guild_id_by_name(&self, server_id: u32, name: String) -> Option<u32>;
  fn get_guild_by_name(&self, server_id: u32, name: String) -> Option<Guild>;
  fn get_guild(&self, guild_id: u32) -> Option<Guild>;
}

impl GetGuild for Armory {
  fn get_guild_id_by_name(&self, server_id: u32, name: String) -> Option<u32> {
    let guilds = self.guilds.read().unwrap();
    guilds.iter().find(|(_, guild)| guild.server_id == server_id && guild.name == name)
      .and_then(|(id, _)| Some(id.clone()))
  }

  fn get_guild_by_name(&self, server_id: u32, name: String) -> Option<Guild> {
    self.get_guild_id_by_name(server_id, name)
        .and_then(|guild_id| self.get_guild(guild_id))
  }

  fn get_guild(&self, guild_id: u32) -> Option<Guild> {
    let guilds = self.guilds.read().unwrap();
    guilds.get(&guild_id).and_then(|guild| Some(guild.clone()))
  }
}