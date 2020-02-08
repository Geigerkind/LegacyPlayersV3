use std::thread;
use std::time::Duration;

use crate::modules::armory_exporter::tools::{RetrieveRecentOfflineCharacters, RetrieveCharacterSkills, RetrieveCharacterItems};
use crate::modules::{ArmoryExporter, CharacterDto};
use crate::Run;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::modules::transport_layer::{CharacterHistoryDto, CharacterInfoDto, CharacterGearDto, CharacterItemDto};
use crate::modules::armory_exporter::domain_value::CharacterItemTable;

impl Run for ArmoryExporter {
  fn run(&mut self) {
    let rate = 60; // TODO: Env
    let sleep_duration_rate = Duration::new(rate, 0);
    loop {
      thread::sleep(sleep_duration_rate);
      println!("Exporting next batch of characters...");

      self.get_recent_offline_characters().iter().for_each(|character_table| {
        println!("Processing {} ({})", character_table.name, character_table.character_id);
        let professions = self.get_profession_skills(character_table.character_id);
        let gear = self.get_character_items(character_table.character_id);

        let character_title;
        if character_table.chosen_title == 0 { character_title = None; } else { character_title = Some(character_table.chosen_title as u16); }
        let _ = self.sender_character.as_ref().unwrap().send((character_table.character_id, CharacterDto {
          server_uid: get_server_uid(character_table.character_id),
          character_history: Some(CharacterHistoryDto {
            character_info: CharacterInfoDto {
              gear: CharacterGearDto {
                head: get_item_slot(0, &gear),
                neck: get_item_slot(1, &gear),
                shoulder: get_item_slot(2, &gear),
                back: get_item_slot(14, &gear),
                chest: get_item_slot(4, &gear),
                shirt: get_item_slot(3, &gear),
                tabard: get_item_slot(18, &gear),
                wrist: get_item_slot(8, &gear),
                main_hand: get_item_slot(15, &gear),
                off_hand: get_item_slot(16, &gear),
                ternary_hand: get_item_slot(17, &gear),
                glove: get_item_slot(9, &gear),
                belt: get_item_slot(5, &gear),
                leg: get_item_slot(6, &gear),
                boot: get_item_slot(7, &gear),
                ring1: get_item_slot(10, &gear),
                ring2: get_item_slot(11, &gear),
                trinket1: get_item_slot(12, &gear),
                trinket2: get_item_slot(13, &gear)
              },
              hero_class_id: character_table.hero_class_id,
              level: character_table.level,
              gender: character_table.gender != 0,
              profession1: professions.get(0).and_then(|skill| Some(skill.skill_id as u16)),
              profession2: professions.get(1).and_then(|skill| Some(skill.skill_id as u16)),
              talent_specialization: None, // TODO
              race_id: character_table.race_id
            },
            character_name: character_table.name.to_owned(),
            character_guild: None, // TODO
            character_title,
            profession_skill_points1: professions.get(0).and_then(|skill| Some(skill.value as u16)),
            profession_skill_points2: professions.get(1).and_then(|skill| Some(skill.value as u16)),
            facial: None // TODO
          })
        }));
      });
    }
  }
}

fn get_server_uid(character_id: u32) -> u64 {
  let salt = "TODO: RANDOM SALT"; // TODO
  let mut hasher = DefaultHasher::new();
  (character_id.to_string() + salt).hash(&mut hasher);
  hasher.finish()
}

// TODO
fn get_item_slot(slot_id: u8, gear: &Vec<CharacterItemTable>) -> Option<CharacterItemDto> {
  None
}