use crate::modules::armory_exporter::domain_value::CharacterTable;
use crate::modules::ArmoryExporter;
use mysql_connection::tools::Select;

pub trait RetrieveRecentOfflineCharacters {
  fn get_recent_offline_characters(&mut self) -> Vec<CharacterTable>;
}

impl RetrieveRecentOfflineCharacters for ArmoryExporter {
  fn get_recent_offline_characters(&mut self) -> Vec<CharacterTable> {
    let last_fetch_time = self.last_fetch_time;
    self.last_fetch_time = time_util::now();
    self.db_characters.select_wparams("SELECT guid, name, race, class, gender, level, chosenTitle, playerBytes, playerBytes2 FROM characters \
      WHERE logout_time < :now AND logout_time > :last_fetch_time", &|mut row| CharacterTable {
      character_id: row.take(0).unwrap(),
      name: row.take(1).unwrap(),
      race_id: row.take(2).unwrap(),
      hero_class_id: row.take(3).unwrap(),
      gender: row.take(4).unwrap(),
      level: row.take(5).unwrap(),
      chosen_title: row.take(6).unwrap(),
      playerbytes1: row.take(7).unwrap(),
      playerbytes2: row.take(8).unwrap()
    }, params!(
      "last_fetch_time" => last_fetch_time,
      "now" => self.last_fetch_time
    )).to_vec()
  }
}