use std::collections::HashMap;

use mysql_connection::material::MySQLConnection;
use mysql_connection::tools::Select;

use crate::modules::data::domain_value::{Expansion, HeroClass, Language, Localization, Profession, Race, Server, Spell, DispelType, PowerType, StatType, SpellEffect, NPC, Icon, Item, Gem, Stat, ItemBonding};
use crate::modules::data::material::Enchant;

#[derive(Debug)]
pub struct Data {
  pub db_main: MySQLConnection,
  pub expansions: HashMap<u8, Expansion>,
  pub languages: HashMap<u8, Language>,
  pub localization: Vec<HashMap<u32, Localization>>,
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
  pub items: Vec<HashMap<u32, Item>>,
  pub gems: Vec<HashMap<u32, Gem>>,
  pub enchants: Vec<HashMap<u32, Enchant>>,
  pub item_bondings: HashMap<u8, ItemBonding>,
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
      icons: HashMap::new(),
      items: Vec::new(),
      gems: Vec::new(),
      enchants: Vec::new(),
      item_bondings: HashMap::new(),
    }
  }
}

impl Data {
  pub fn init(mut self, debug_collection: Option<u8>) -> Self
  {
    let init_flag = debug_collection.unwrap_or(0);

    if self::Data::should_init(init_flag, 1) { self.expansions.init(&self.db_main); }
    if self::Data::should_init(init_flag, 2) { self.languages.init(&self.db_main); }
    if self::Data::should_init(init_flag, 3) { self.localization.init(&self.db_main); }
    if self::Data::should_init(init_flag, 4) { self.races.init(&self.db_main); }
    if self::Data::should_init(init_flag, 5) { self.professions.init(&self.db_main); }
    if self::Data::should_init(init_flag, 6) { self.servers.init(&self.db_main); }
    if self::Data::should_init(init_flag, 7) { self.hero_classes.init(&self.db_main); }
    if self::Data::should_init(init_flag, 8) { self.spells.init(&self.db_main); }
    if self::Data::should_init(init_flag, 9) { self.dispel_types.init(&self.db_main); }
    if self::Data::should_init(init_flag, 10) { self.power_types.init(&self.db_main); }
    if self::Data::should_init(init_flag, 11) { self.stat_types.init(&self.db_main); }
    if self::Data::should_init(init_flag, 12) { self.spell_effects.init(&self.db_main); }
    if self::Data::should_init(init_flag, 13) { self.npcs.init(&self.db_main); }
    if self::Data::should_init(init_flag, 14) { self.icons.init(&self.db_main); }
    if self::Data::should_init(init_flag, 15) { self.items.init(&self.db_main); }
    if self::Data::should_init(init_flag, 16) { self.gems.init(&self.db_main); }
    if self::Data::should_init(init_flag, 17) { self.enchants.init(&self.db_main); }
    if self::Data::should_init(init_flag, 18) { self.item_bondings.init(&self.db_main); }
    self
  }

  fn should_init(init_flag: u8, trigger_flag: u8) -> bool {
    return init_flag == 0 || init_flag == trigger_flag;
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

impl Init for Vec<HashMap<u32, Localization>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_language_id = 0;
    db.select("SELECT * FROM data_localization ORDER BY language_id, id", &|mut row| {
      Localization {
        language_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        content: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| {
      if result.language_id != last_language_id {
        self.push(HashMap::new());
        last_language_id = result.language_id;
      }
      self.get_mut(result.language_id as usize - 1).unwrap().insert(result.id, result.to_owned());
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
    let mut last_expansion_id = 0;
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
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
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

impl Init for Vec<HashMap<u32, Item>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_item ORDER BY expansion_id, id", &|mut row| {
      Item {
        expansion_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        localization_id: row.take(2).unwrap(),
        icon: row.take(3).unwrap(),
        quality: row.take(4).unwrap(),
        inventory_type: row.take(5).unwrap(),
        class_id: row.take(6).unwrap(),
        required_level: row.take(7).unwrap(),
        bonding: row.take_opt(8).unwrap().ok(),
        sheath: row.take_opt(9).unwrap().ok(),
        itemset: row.take_opt(10).unwrap().ok(),
        max_durability: row.take(11).unwrap(),
        item_level: row.take(12).unwrap(),
        delay: row.take(13).unwrap(),
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

impl Init for Vec<HashMap<u32, Gem>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_gem ORDER BY expansion_id, item_id", &|mut row| {
      Gem {
        expansion_id: row.take(0).unwrap(),
        item_id: row.take(1).unwrap(),
        enchant_id: row.take(2).unwrap(),
        flag: row.take(3).unwrap()
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      self.get_mut(result.expansion_id as usize - 2).unwrap().insert(result.item_id, result.to_owned());
    });
  }
}

impl Init for Vec<HashMap<u32, Enchant>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_enchant ORDER BY expansion_id, id", &|mut row| {
      let mut stats = Vec::new();
      for i in (3..8).step_by(2) {
        let stat_type = row.take_opt(i).unwrap().ok();
        let stat_value = row.take_opt(i+1).unwrap().ok();
        if stat_type.is_none() {
          break;
        }
        stats.push(Stat {
          stat_type: stat_type.unwrap(),
          stat_value: stat_value.unwrap()
        });
      }
      Enchant {
        expansion_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        localization_id: row.take(2).unwrap(),
        stats
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

impl Init for HashMap<u8, ItemBonding> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_item_bonding", &|mut row| {
      ItemBonding {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}