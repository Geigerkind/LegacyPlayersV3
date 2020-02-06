use std::cmp::Ordering;

use crate::modules::armory::Armory;
use crate::modules::armory::dto::{CharacterSearchFilter, CharacterSearchResult, SearchResult, CharacterSearchGuildDto, CharacterSearchCharacterDto};
use crate::modules::armory::tools::GetGuild;
use crate::modules::data::Data;
use crate::modules::data::tools::RetrieveRace;

pub trait PerformCharacterSearch {
  fn get_character_search_result(&self, data: &Data, filter: CharacterSearchFilter) -> SearchResult<CharacterSearchResult>;
}

impl PerformCharacterSearch for Armory {
  fn get_character_search_result(&self, data: &Data, filter: CharacterSearchFilter) -> SearchResult<CharacterSearchResult> {
    let mut filter = filter.to_owned();
    let characters = self.characters.read().unwrap();
    if filter.name.filter.is_some() { filter.name.filter = filter.name.filter.replace(filter.name.filter.as_ref().unwrap().to_lowercase()); }
    if filter.guild.filter.is_some() { filter.guild.filter = filter.guild.filter.replace(filter.guild.filter.as_ref().unwrap().to_lowercase()); }
    let intermediate = characters.iter()
      .filter(|(_, character)| character.last_update.is_some())
      .filter(|(_, character)| filter.server.filter.is_none() || filter.server.filter.contains(&character.server_id))
      .filter(|(_, character)| filter.name.filter.is_none() || character.last_update.as_ref().unwrap().character_name.to_lowercase().contains(filter.name.filter.as_ref().unwrap()))
      .filter(|(_, character)| filter.hero_class.filter.is_none() || filter.hero_class.filter.contains(&character.last_update.as_ref().unwrap().character_info.hero_class_id))
      .filter(|(_, character)| {
        if filter.last_updated.filter.is_none() {
          return true;
        }
        let filter_timestamp = filter.last_updated.filter.as_ref().unwrap().clone();
        let current_timestamp = character.last_update.as_ref().unwrap().timestamp;
        return current_timestamp >= filter_timestamp && current_timestamp <= filter_timestamp + 24 * 60 * 60;
      })
      // This is very expensive
      .filter(|(_, character)| {
        if filter.guild.filter.is_none() {
          return true;
        }
        if let Some(character_guild) = character.last_update.as_ref().unwrap().character_guild.as_ref() {
          if let Some(guild) = self.get_guild(character_guild.guild_id) {
            return guild.name.contains(filter.guild.filter.as_ref().unwrap());
          }
        }
        return false;
      });
    let num_characters = intermediate.clone().count();

    // This is very inefficient!
    let mut result: Vec<CharacterSearchResult> = intermediate
      .skip((filter.page * 10) as usize)
      .map(|(_, character)| {
        let race_id = character.last_update.as_ref().unwrap().character_info.race_id;
        CharacterSearchResult {
          faction: data.get_race(race_id).unwrap().faction,
          guild: character.last_update.as_ref().unwrap().character_guild.as_ref()
            .and_then(|character_guild| self.get_guild(character_guild.guild_id)
              .and_then(|guild| Some(CharacterSearchGuildDto {
                guild_id: guild.id,
                name: guild.name.to_owned()
              }))),
          character: CharacterSearchCharacterDto {
            character_id: character.id,
            name: character.last_update.as_ref().unwrap().character_name.clone(),
            hero_class_id: character.last_update.as_ref().unwrap().character_info.hero_class_id,
            server_id: character.server_id
          },
          timestamp: character.last_update.as_ref().unwrap().timestamp
        }
      })
      .collect();
    result.sort_by(|left: &CharacterSearchResult, right: &CharacterSearchResult| {
      if let Some(sorting) = filter.hero_class.sorting {
        let ordering = left.character.hero_class_id.cmp(&right.character.hero_class_id);
        if ordering != Ordering::Equal {
          return negate_ordering(ordering, sorting);
        }
      }

      if let Some(sorting) = filter.name.sorting {
        let ordering = left.character.name.cmp(&right.character.name);
        if ordering != Ordering::Equal {
          return negate_ordering(ordering, sorting);
        }
      }

      if let Some(sorting) = filter.guild.sorting {
        if left.guild.is_some() && right.guild.is_some() {
          let ordering = left.guild.as_ref().unwrap().name.cmp(&right.guild.as_ref().unwrap().name);
          if ordering != Ordering::Equal {
            return negate_ordering(ordering, sorting);
          }
        }
      }

      if let Some(sorting) = filter.server.sorting {
        let ordering = left.character.server_id.cmp(&right.character.server_id);
        if ordering != Ordering::Equal {
          return negate_ordering(ordering, sorting);
        }
      }

      if let Some(sorting) = filter.last_updated.sorting {
        let ordering = left.timestamp.cmp(&right.timestamp);
        if ordering != Ordering::Equal {
          return negate_ordering(ordering, sorting);
        }
      }

      return Ordering::Equal;
    });
    SearchResult {
      result: result.iter().take(10).map(|cs| cs.to_owned()).collect::<Vec<CharacterSearchResult>>(),
      num_items: num_characters
    }
  }
}

fn negate_ordering(ordering: Ordering, sorting: bool) -> Ordering {
  if ordering == Ordering::Less {
    if sorting {
      return Ordering::Less;
    }
    return Ordering::Greater;
  }
  if sorting {
    return Ordering::Greater;
  }
  return Ordering::Less;
}