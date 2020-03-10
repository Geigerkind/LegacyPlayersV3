use mysql_connection::tools::Execute;

use crate::modules::armory::dto::ArmoryFailure;
use crate::modules::armory::tools::GetGuild;
use crate::modules::armory::Armory;

pub trait DeleteGuild {
    fn delete_guild(&self, id: u32) -> Result<(), ArmoryFailure>;
    fn delete_guild_by_uid(&self, server_id: u32, uid: u64) -> Result<(), ArmoryFailure>;
}

impl DeleteGuild for Armory {
    fn delete_guild(&self, id: u32) -> Result<(), ArmoryFailure> {
        let mut guilds = self.guilds.write().unwrap();
        if self.db_main.execute_wparams(
            "DELETE FROM armory_guild WHERE id=:id",
            params!(
              "id" => id.clone()
            ),
        ) {
            return guilds
                .remove(&id)
                .ok_or(ArmoryFailure::Database("Invalid guild id o.O".to_owned()))
                .and_then(|_| Ok(()));
        }
        Err(ArmoryFailure::Database("delete_guild".to_owned()))
    }

    fn delete_guild_by_uid(&self, server_id: u32, uid: u64) -> Result<(), ArmoryFailure> {
        self.get_guild_id_by_uid(server_id, uid)
            .ok_or(ArmoryFailure::InvalidInput)
            .and_then(|id| self.delete_guild(id))
    }
}
