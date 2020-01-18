use crate::modules::armory::dto::GuildDto;
use crate::modules::armory::Armory;
use crate::modules::armory::tools::{CreateGuild, GetGuild};
use mysql_connection::tools::Execute;

#[test]
fn create_guild() {
  let armory = Armory::default();
  let guild_dto = GuildDto {
    server_uid: 23423214,
    name: "WeirdGuildName".to_owned()
  };

  let guild_res = armory.create_guild(1, guild_dto.clone());
  assert!(guild_res.is_ok());

  let guild = guild_res.unwrap();
  assert!(guild.compare_by_value(&guild_dto));

  let guild_res2 = armory.get_guild(guild.id);
  assert!(guild_res2.is_some());

  let guild2 = guild_res2.unwrap();
  assert!(guild2.deep_eq(&guild));

  armory.db_main.execute_wparams("DELETE FROM armory_guild WHERE id=:id", params!("id" => guild.id));
}