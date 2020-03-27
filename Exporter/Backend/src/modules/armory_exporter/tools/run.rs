use std::ops::Shr;
use std::{thread, env};
use std::time::Duration;

use crate::modules::{ArmoryExporter, CharacterDto};
use crate::modules::armory_exporter::domain_value::CharacterItemTable;
use crate::modules::armory_exporter::tools::{RetrieveCharacterGuild, RetrieveCharacterItems, RetrieveCharacterSkills, RetrieveRecentOfflineCharacters, UpdateMetaData, RetrieveCharacterTalents, RetrieveCharacterArenaTeams};
use crate::modules::transport_layer::{CharacterFacialDto, CharacterGearDto, CharacterGuildDto, CharacterHistoryDto, CharacterInfoDto, CharacterItemDto, GuildDto, GuildRank};
use crate::Run;
use std::collections::HashMap;
use crate::modules::util::{salt_u32_u64, salt_u64_u64};

impl Run for ArmoryExporter {
  fn run(&mut self) {
    let rate = env::var("CHARACTER_FETCH_INTERVAL_IN_SEC").unwrap().parse::<u64>().unwrap();
    let sleep_duration_rate = Duration::new(rate, 0);
    loop {
      thread::sleep(sleep_duration_rate);
      println!("Exporting next batch of characters...");
      let offline_characters= self.get_recent_offline_characters();
      if !offline_characters.is_empty() {
        self.last_fetch_time = time_util::now();
      }
      offline_characters.iter().for_each(|character_table| {
        println!("Processing {} ({})", character_table.name, character_table.character_id);
        let professions = self.get_profession_skills(character_table.character_id);
        let gear = self.get_character_items(character_table.character_id);
        let guild = self.get_character_guild(character_table.character_id);
        let talent = self.get_character_talent(character_table.character_id);
        let mut arena_teams = self.get_arena_teams(character_table.character_id);
        arena_teams.iter_mut().for_each(|team| team.team_id = salt_u64_u64(team.team_id));

        let character_title;
        if character_table.chosen_title == 0 { character_title = None; } else { character_title = Some(character_table.chosen_title as u16); }
        let _ = self.sender_character.as_ref().unwrap().send((character_table.character_id, CharacterDto {
          server_uid: salt_u32_u64(character_table.character_id),
          character_history: Some(CharacterHistoryDto {
            character_info: CharacterInfoDto {
              gear: CharacterGearDto {
                head: get_item_slot(0, &gear, &self.gem_enchant_id_to_item_id),
                neck: get_item_slot(1, &gear, &self.gem_enchant_id_to_item_id),
                shoulder: get_item_slot(2, &gear, &self.gem_enchant_id_to_item_id),
                back: get_item_slot(14, &gear, &self.gem_enchant_id_to_item_id),
                chest: get_item_slot(4, &gear, &self.gem_enchant_id_to_item_id),
                shirt: get_item_slot(3, &gear, &self.gem_enchant_id_to_item_id),
                tabard: get_item_slot(18, &gear, &self.gem_enchant_id_to_item_id),
                wrist: get_item_slot(8, &gear, &self.gem_enchant_id_to_item_id),
                main_hand: get_item_slot(15, &gear, &self.gem_enchant_id_to_item_id),
                off_hand: get_item_slot(16, &gear, &self.gem_enchant_id_to_item_id),
                ternary_hand: get_item_slot(17, &gear, &self.gem_enchant_id_to_item_id),
                glove: get_item_slot(9, &gear, &self.gem_enchant_id_to_item_id),
                belt: get_item_slot(5, &gear, &self.gem_enchant_id_to_item_id),
                leg: get_item_slot(6, &gear, &self.gem_enchant_id_to_item_id),
                boot: get_item_slot(7, &gear, &self.gem_enchant_id_to_item_id),
                ring1: get_item_slot(10, &gear, &self.gem_enchant_id_to_item_id),
                ring2: get_item_slot(11, &gear, &self.gem_enchant_id_to_item_id),
                trinket1: get_item_slot(12, &gear, &self.gem_enchant_id_to_item_id),
                trinket2: get_item_slot(13, &gear, &self.gem_enchant_id_to_item_id),
              },
              hero_class_id: character_table.hero_class_id,
              level: character_table.level,
              gender: character_table.gender != 0,
              profession1: professions.get(0).and_then(|skill| Some(skill.skill_id as u16)),
              profession2: professions.get(1).and_then(|skill| Some(skill.skill_id as u16)),
              talent_specialization: Some(talent),
              race_id: character_table.race_id,
            },
            character_name: character_table.name.to_owned(),
            character_guild: guild.and_then(|char_guild_table| Some(CharacterGuildDto {
              guild: GuildDto {
                server_uid: salt_u32_u64(char_guild_table.guild_id),
                name: char_guild_table.guild_name.to_owned(),
              },
              rank: GuildRank {
                index: char_guild_table.rank_index,
                name: char_guild_table.rank_name.to_owned()
              },
            })),
            character_title,
            profession_skill_points1: professions.get(0).and_then(|skill| Some(skill.value as u16)),
            profession_skill_points2: professions.get(1).and_then(|skill| Some(skill.value as u16)),
            facial: Some(CharacterFacialDto {
              skin_color: (character_table.playerbytes1 % 256 as u32) as u8,
              face_style: (character_table.playerbytes1.shr(8) % 256 as u32) as u8,
              hair_style: (character_table.playerbytes1.shr(16) % 256 as u32) as u8,
              hair_color: (character_table.playerbytes1.shr(24) % 256 as u32) as u8,
              facial_hair: (character_table.playerbytes2 % 256 as u32) as u8,
            }),
            arena_teams
          }),
        }));
      });

      self.update_meta_data();
    }
  }
}

fn get_item_slot(slot_id: u32, gear: &Vec<CharacterItemTable>, enchant_id_to_item_id: &HashMap<u32, u32>) -> Option<CharacterItemDto> {
  gear.iter().find(|item| item.slot == slot_id)
    .and_then(|char_item_table| {
      let enchant_id;
      if char_item_table.enchant_ids[0] == 0 { enchant_id = None; } else { enchant_id = Some(char_item_table.enchant_ids[0]); }
      let random_property_id;
      if char_item_table.random_property_id == 0 { random_property_id = None; } else { random_property_id = Some(char_item_table.random_property_id); }
      let mut gem_ids = Vec::new();
      for i in 2..6 {
        if char_item_table.enchant_ids[i] == 0 {
          gem_ids.push(None);
        } else if enchant_id_to_item_id.contains_key(&char_item_table.enchant_ids[i]) {
          gem_ids.push(Some(enchant_id_to_item_id.get(&char_item_table.enchant_ids[i]).unwrap().clone()))
        }
      }

      Some(CharacterItemDto {
        item_id: char_item_table.item_id,
        random_property_id,
        enchant_id,
        gem_ids
      })
    })
}