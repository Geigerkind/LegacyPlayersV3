use crate::modules::ArmoryExporter;
use crate::modules::transport_layer::ArenaTeam;
use mysql_connection::tools::Select;
use std::env;

pub trait RetrieveCharacterArenaTeams {
  fn get_arena_teams(&self, character_id: u32) -> Vec<ArenaTeam>;
}

impl RetrieveCharacterArenaTeams for ArmoryExporter {
  fn get_arena_teams(&self, character_id: u32) -> Vec<ArenaTeam> {
    lazy_static! {
      static ref EXPANSION_ID: u8 = env::var("EXPANSION_ID").unwrap().parse::<u8>().unwrap();
    }
    if *EXPANSION_ID <= 2 {
      return Vec::new();
    }

    self.db_characters.select_wparams(
      "SELECT atm.arenaTeamId, art.name,  art.type, art.rating, atm.personalRating FROM arena_team_member atm\
       JOIN arena_team art ON atm.arenaTeamId = art.arenaTeamId WHERE atm.guid = :character_id", &|mut row|
      ArenaTeam {
        team_id: row.take(0).unwrap(),
        name: row.take(1).unwrap(),
        team_type: row.take(2).unwrap(),
        team_rating: row.take(3).unwrap(),
        personal_rating: row.take(4).unwrap()
      }, params!(
      "character_id" => character_id
    ))
  }
}