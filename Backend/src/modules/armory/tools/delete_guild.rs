use mysql_connection::tools::Execute;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::tools::GetGuild;

pub trait DeleteGuild {
  fn delete_guild(&self, id: u32) -> Result<(), Failure>;
  fn delete_guild_by_uid(&self, server_id: u32, uid: u64) -> Result<(), Failure>;
}

impl DeleteGuild for Armory {
  fn delete_guild(&self, id: u32) -> Result<(), Failure> {
    let mut guilds = self.guilds.write().unwrap();
    if self.db_main.execute_wparams("DELETE FROM armory_guild WHERE id=:id", params!(
      "id" => id.clone()
    )) {
      return guilds.remove(&id).ok_or(Failure::Unknown).and_then(|_| Ok(()));
    }
    Err(Failure::InvalidInput)
  }

  fn delete_guild_by_uid(&self, server_id: u32, uid: u64) -> Result<(), Failure> {
    self.get_guild_id_by_uid(server_id, uid).ok_or(Failure::InvalidInput).and_then(|id| self.delete_guild(id))
  }
}