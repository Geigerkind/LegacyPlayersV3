use std::collections::HashMap;

use mysql_connection::material::MySQLConnection;
use mysql_connection::tools::Select;

use crate::modules::data::domain_value::{Expansion, HeroClass, Language, Localization, Profession, Race, Server, Spell, DispelType, PowerType, StatType, SpellEffect, NPC, Icon};

#[derive(Debug)]
pub struct Data {
  pub db_main: MySQLConnection,
  pub expansions: HashMap<u8, Expansion>,
  pub languages: HashMap<u8, Language>,
  pub localization: Vec<HashMap<u32, String>>,
  pub races: HashMap<u8, Race>,
  pub professions: HashMap<u8, Profession>,
  pub servers: HashMap<u32, Server>,
  pub hero_classes: HashMap<u8, HeroClass>,
  pub spells: Vec<HashMap<u32, Spell>>,
  pub dispel_types: HashMap<u8, DispelType>,
  pub power_types: HashMap<u8, PowerType>,
  pub stat_types: HashMap<u8, StatType>,
  pub spell_effects: Vec<HashMap<u32, Vec<SpellEffect>>>,
  pub npcs: Vec<HashMap<u32, NPC>>,
  pub icons: HashMap<u16, Icon>,
}

impl Default for Data {
  fn default() -> Self
  {
    Data {
      db_main: MySQLConnection::new("main"),
      expansions: HashMap::new(),
      languages: HashMap::new(),
      localization: Vec::new(),
      races: HashMap::new(),
      professions: HashMap::new(),
      servers: HashMap::new(),
      hero_classes: HashMap::new(),
      spells: Vec::new(),
      dispel_types: HashMap::new(),
      power_types: HashMap::new(),
      stat_types: HashMap::new(),
      spell_effects: Vec::new(),
      npcs: Vec::new(),
      icons: HashMap::new()
    }
  }
}

impl Data {
  pub fn init(mut self) -> Self
  {
    self.expansions.init(&self.db_main);
    self.languages.init(&self.db_main);
    for _i in 0..self.languages.len() {
      self.localization.push(HashMap::new());
    }
    self.localization.init(&self.db_main);
    self.races.init(&self.db_main);
    self.professions.init(&self.db_main);
    self.servers.init(&self.db_main);
    self.hero_classes.init(&self.db_main);
    for _i in 0..self.expansions.len() {
      self.spells.push(HashMap::new());
    }
    self.spells.init(&self.db_main);
    self.dispel_types.init(&self.db_main);
    self.power_types.init(&self.db_main);
    self.stat_types.init(&self.db_main);
    self.spell_effects.init(&self.db_main);
    self.npcs.init(&self.db_main);
    self.icons.init(&self.db_main);
    self
  }
}


// Initializer for the collections
trait Init {
  fn init(&mut self, db: &MySQLConnection);
}

impl Init for HashMap<u8, Expansion> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_expansion", &|mut row| {
      Expansion {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, Language> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_language", &|mut row| {
      Language {
        id: row.take(0).unwrap(),
        name: row.take(1).unwrap(),
        short_code: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for Vec<HashMap<u32, String>> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_localization ORDER BY language_id, id", &|mut row| {
      Localization {
        language_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        content: row.take(2).unwrap(),
      }
    }).iter().for_each(|localization| {
      self.get_mut(localization.language_id as usize - 1).unwrap().insert(localization.id, localization.content.to_owned());
    });
  }
}

impl Init for HashMap<u8, Race> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_race", &|mut row| {
      Race {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, Profession> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_profession", &|mut row| {
      Profession {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u32, Server> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_server", &|mut row| {
      Server {
        id: row.take(0).unwrap(),
        expansion_id: row.take(1).unwrap(),
        name: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, HeroClass> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_hero_class", &|mut row| {
      HeroClass {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
        color: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for Vec<HashMap<u32, Spell>> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_spell ORDER BY expansion_id, id", &|mut row| {
      Spell {
        expansion_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        localization_id: row.take(2).unwrap(),
        subtext_localization_id: row.take(3).unwrap(),
        cost: row.take(4).unwrap(),
        cost_in_percent: row.take(5).unwrap(),
        power_type: row.take(6).unwrap(),
        cast_time: row.take(7).unwrap(),
        school_mask: row.take(8).unwrap(),
        dispel_type: row.take(9).unwrap(),
        range_max: row.take(10).unwrap(),
        cooldown: row.take(11).unwrap(),
        duration: row.take(12).unwrap(),
        icon: row.take(13).unwrap(),
        description_localization_id: row.take(14).unwrap(),
        aura_localization_id: row.take(15).unwrap()
      }
    }).iter().for_each(|result| {
      self.get_mut(result.expansion_id as usize - 1).unwrap().insert(result.id, result.to_owned());
    });
  }
}

impl Init for HashMap<u8, DispelType> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_spell_dispel_type", &|mut row| {
      DispelType {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
        color: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, PowerType> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_spell_power_type", &|mut row| {
      PowerType {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
        color: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, StatType> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_stat_type", &|mut row| {
      StatType {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for Vec<HashMap<u32, Vec<SpellEffect>>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    let mut last_spell_id = 0;
    db.select("SELECT * FROM data_spell_effect ORDER BY expansion_id, spell_id, id", &|mut row| {
      SpellEffect {
        id: row.take(0).unwrap(),
        expansion_id: row.take(1).unwrap(),
        spell_id: row.take(2).unwrap(),
        points_lower: row.take(3).unwrap(),
        points_upper: row.take(4).unwrap(),
        chain_targets: row.take(5).unwrap(),
        radius: row.take(6).unwrap(),
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      let expansion_vec = self.get_mut(result.expansion_id as usize - 1).unwrap();
      if result.spell_id != last_spell_id {
        expansion_vec.insert(result.spell_id, Vec::new());
        last_spell_id = result.spell_id;
      }
      expansion_vec.get_mut(&result.spell_id).unwrap().push(result.to_owned());
    });
  }
}

impl Init for Vec<HashMap<u32, NPC>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_npc ORDER BY expansion_id, id", &|mut row| {
      NPC {
        expansion_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        localization_id: row.take(2).unwrap(),
        is_boss: row.take(3).unwrap(),
        friend: row.take(4).unwrap(),
        family: row.take(5).unwrap(),
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      self.get_mut(result.expansion_id as usize - 1).unwrap().insert(result.id, result.to_owned());
    });
  }
}

impl Init for HashMap<u16, Icon> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_icon ORDER BY id", &|mut row| {
      Icon {
        id: row.take(0).unwrap(),
        name: row.take(1).unwrap(),
      }
    }).iter().for_each(|icon| {
      self.insert(icon.id, icon.to_owned());
    });
  }
}