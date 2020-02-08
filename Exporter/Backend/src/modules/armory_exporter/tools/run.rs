use std::thread;
use std::time::Duration;

use crate::modules::armory_exporter::tools::{RetrieveRecentOfflineCharacters, RetrieveCharacterSkills, RetrieveCharacterItems};
use crate::modules::ArmoryExporter;
use crate::Run;

impl Run for ArmoryExporter {
  fn run(&mut self) {
    loop {
      thread::sleep(Duration::new(1, 0));
      println!("Exporting next batch of characters...");

      self.get_recent_offline_characters().iter().for_each(|character_table| {
        println!("Processing {} ({})", character_table.name, character_table.character_id);
        let professions = self.get_profession_skills(character_table.character_id);
        let gear = self.get_character_items(character_table.character_id);
        // TODO: Talents
      });
    }
  }
}