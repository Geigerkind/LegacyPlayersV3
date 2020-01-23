use std::collections::HashMap;
use std::sync::RwLock;

use mysql_connection::material::MySQLConnection;
use mysql_connection::tools::Select;

use crate::modules::armory::domain_value::{CharacterGear, CharacterInfo, CharacterItem, CharacterGuild};
use crate::modules::armory::material::{Character, CharacterHistory, Guild};

#[derive(Debug)]
pub struct Armory {
  pub db_main: MySQLConnection,
  pub characters: RwLock<HashMap<u32, Character>>,
  pub guilds: RwLock<HashMap<u32, Guild>>,
}

impl Default for Armory {
  fn default() -> Self
  {
    Armory {
      db_main: MySQLConnection::new("main"),
      characters: RwLock::new(HashMap::new()),
      guilds: RwLock::new(HashMap::new()),
    }
  }
}

impl Armory {
  pub fn init(self) -> Self
  {
    self.characters.write().unwrap().init(&self.db_main);
    self.guilds.write().unwrap().init(&self.db_main);
    self
  }
}

trait Init {
  fn init(&mut self, db: &MySQLConnection);
}

impl Init for HashMap<u32, Character> {
  fn init(&mut self, db: &MySQLConnection) {
    // Loading the character itself
    db.select("SELECT * FROM armory_character", &|mut row| {
      Character {
        id: row.take(0).unwrap(),
        server_id: row.take(1).unwrap(),
        server_uid: row.take(2).unwrap(),
        last_update: None,
        history_ids: Vec::new()
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });

    // Loading the history_ids
    db.select("SELECT id, character_id FROM armory_character_history ORDER BY id", &|mut row| {
      let id: u32 = row.take(0).unwrap();
      let character_id: u32 = row.take(1).unwrap();
      (id, character_id)
    }).iter().for_each(|result| { self.get_mut(&result.1).unwrap().history_ids.push(result.0); });

    // Load the actual newest character history data
    db.select("SELECT ach.*, aci.*, ag.id, ai1.*, ai2.*, ai3.*, ai4.*, ai5.*, ai6.*, ai7.*, ai8.*, ai9.*, ai10.*, ai11.*, ai12.*, ai13.*, ai14.*, ai15.*, ai16.*, ai17.*, ai18.*, ai19.* FROM armory_character_history ach JOIN (SELECT MAX(id) id FROM armory_character_history GROUP BY character_id) ach_max ON ach.id = ach_max.id JOIN armory_character_info aci ON ach.character_info_id = aci.id JOIN armory_gear ag ON aci.gear_id = ag.id LEFT JOIN armory_item ai1 ON ag.head = ai1.id LEFT JOIN armory_item ai2 ON ag.neck = ai2.id LEFT JOIN armory_item ai3 ON ag.shoulder = ai3.id LEFT JOIN armory_item ai4 ON ag.back = ai4.id LEFT JOIN armory_item ai5 ON ag.chest = ai5.id LEFT JOIN armory_item ai6 ON ag.shirt = ai6.id LEFT JOIN armory_item ai7 ON ag.tabard = ai7.id LEFT JOIN armory_item ai8 ON ag.wrist = ai8.id LEFT JOIN armory_item ai9 ON ag.main_hand = ai9.id LEFT JOIN armory_item ai10 ON ag.off_hand = ai10.id LEFT JOIN armory_item ai11 ON ag.ternary_hand = ai11.id LEFT JOIN armory_item ai12 ON ag.glove = ai12.id LEFT JOIN armory_item ai13 ON ag.belt = ai13.id LEFT JOIN armory_item ai14 ON ag.leg = ai14.id LEFT JOIN armory_item ai15 ON ag.boot = ai15.id LEFT JOIN armory_item ai16 ON ag.ring1 = ai16.id LEFT JOIN armory_item ai17 ON ag.ring2 = ai17.id LEFT JOIN armory_item ai18 ON ag.trinket1 = ai18.id LEFT JOIN armory_item ai19 ON ag.trinket2 = ai19.id", &|mut row| {
      let mut gear_slots: Vec<Option<CharacterItem>> = Vec::new();
      for i in (20..172).step_by(8) {
        let id = row.take_opt(i).unwrap().ok();
        if id.is_none() {
          gear_slots.push(None);
          continue;
        }

        gear_slots.push(Some(CharacterItem {
          id: id.unwrap(),
          item_id: row.take(i+1).unwrap(),
          random_property_id: row.take_opt(i+2).unwrap().ok(),
          enchant_id: row.take_opt(i+3).unwrap().ok(),
          gem_ids: vec![
            row.take_opt(i+4).unwrap().ok(),
            row.take_opt(i+5).unwrap().ok(),
            row.take_opt(i+6).unwrap().ok(),
            row.take_opt(i+7).unwrap().ok()
          ]
        }));
      }

      CharacterHistory {
        id: row.take(0).unwrap(),
        character_id: row.take(1).unwrap(),
        character_name: row.take(3).unwrap(),
        character_guild: row.take_opt(4).unwrap().ok().and_then(|guild_id| Some(CharacterGuild {
          guild_id,
          rank: row.take(5).unwrap()
        })),
        character_title: row.take_opt(6).unwrap().ok(),
        profession_skill_points1: row.take_opt(7).unwrap().ok(),
        profession_skill_points2: row.take_opt(8).unwrap().ok(),
        timestamp: row.take(9).unwrap(),
        character_info: CharacterInfo {
          id: row.take(10).unwrap(),
          hero_class_id: row.take(12).unwrap(),
          level: row.take(13).unwrap(),
          gender: row.take(14).unwrap(),
          profession1: row.take_opt(15).unwrap().ok(),
          profession2: row.take_opt(16).unwrap().ok(),
          talent_specialization: row.take_opt(17).unwrap().ok(),
          race_id: row.take(18).unwrap(),
          gear: CharacterGear {
            id: row.take(19).unwrap(),
            trinket2: gear_slots.pop().unwrap(),
            trinket1: gear_slots.pop().unwrap(),
            ring2: gear_slots.pop().unwrap(),
            ring1: gear_slots.pop().unwrap(),
            boot: gear_slots.pop().unwrap(),
            leg: gear_slots.pop().unwrap(),
            belt: gear_slots.pop().unwrap(),
            glove: gear_slots.pop().unwrap(),
            ternary_hand: gear_slots.pop().unwrap(),
            off_hand: gear_slots.pop().unwrap(),
            main_hand: gear_slots.pop().unwrap(),
            wrist: gear_slots.pop().unwrap(),
            tabard: gear_slots.pop().unwrap(),
            shirt: gear_slots.pop().unwrap(),
            chest: gear_slots.pop().unwrap(),
            back: gear_slots.pop().unwrap(),
            shoulder: gear_slots.pop().unwrap(),
            neck: gear_slots.pop().unwrap(),
            head: gear_slots.pop().unwrap(),
          }
        }
      }
    }).iter().for_each(|result| { self.get_mut(&result.character_id).unwrap().last_update = Some(result.to_owned()) });
  }
}

impl Init for HashMap<u32, Guild> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM armory_guild", &|mut row| {
      Guild {
        id: row.take(0).unwrap(),
        server_uid: row.take(1).unwrap(),
        server_id: row.take(2).unwrap(),
        name: row.take(3).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}