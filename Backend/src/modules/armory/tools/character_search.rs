use crate::modules::armory::dto::{CharacterSearchResult, CharacterSearchFilter};
use crate::modules::data::Data;
use crate::modules::armory::Armory;
use crate::dto::SelectOption;
use crate::modules::data::tools::{RetrieveLocalization, RetrieveRace, RetrieveServer, RetrieveHeroClass};
use std::cmp::Ordering;
use crate::modules::armory::tools::GetGuild;

pub trait PerformCharacterSearch {
  fn get_character_search_result(&self, data: &Data, language_id: u8, filter: CharacterSearchFilter) -> Vec<CharacterSearchResult>;
}

impl PerformCharacterSearch for Armory {
  fn get_character_search_result(&self, data: &Data, language_id: u8, filter: CharacterSearchFilter) -> Vec<CharacterSearchResult> {
    let characters = self.characters.read().unwrap();
    let mut result: Vec<CharacterSearchResult> = characters.iter()
      .filter(|(_, character)| character.last_update.is_some())
      .filter(|(_, character)| filter.server.filter.is_none() || filter.server.filter.contains(&character.server_id))
      .filter(|(_, character)| filter.name.filter.is_none() || character.last_update.as_ref().unwrap().character_name.contains(filter.name.filter.as_ref().unwrap()))
      .filter(|(_, character)| filter.race.filter.is_none() || filter.race.filter.contains(&character.last_update.as_ref().unwrap().character_info.race_id))
      .filter(|(_, character)| filter.gender.filter.is_none() || ((*filter.gender.filter.as_ref().unwrap()) != 0) == character.last_update.as_ref().unwrap().character_info.gender)
      .filter(|(_, character)| filter.hero_class.filter.is_none() || filter.hero_class.filter.contains(&character.last_update.as_ref().unwrap().character_info.hero_class_id))
      // TODO
      .filter(|(_, character)| filter.last_updated.filter.is_none() || filter.last_updated.filter.contains(&character.last_update.as_ref().unwrap().timestamp))
      .filter(|(_, character)| filter.faction.filter.is_none() || data.get_race(character.last_update.as_ref().unwrap().character_info.race_id).unwrap().faction as u8 == *filter.faction.filter.as_ref().unwrap())
      // This is also very expensive
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
      })
      .take(10)
      .map(|(_, character)| {
        let gender = character.last_update.as_ref().unwrap().character_info.gender as u8;
        let race_id = character.last_update.as_ref().unwrap().character_info.race_id;
        let hero_class_id = character.last_update.as_ref().unwrap().character_info.hero_class_id;
        let faction = data.get_race(race_id).unwrap().faction as u8;
        CharacterSearchResult {
          gender: SelectOption { label_key: "General.gender_".to_owned() + &gender.to_string(), value: gender },
          race: SelectOption { label_key: data.get_localization(language_id, data.get_race(race_id).unwrap().localization_id).unwrap().content.to_owned(), value: race_id },
          faction: SelectOption { label_key: "General.faction_".to_owned() + &faction.to_string(), value: faction },
          server: SelectOption { label_key: data.get_server(character.server_id).unwrap().name.to_owned(), value: character.server_id },
          hero_class: SelectOption { label_key: data.get_localization(language_id, data.get_hero_class(hero_class_id).unwrap().localization_id).unwrap().content.to_owned(), value: hero_class_id },
          name: character.last_update.as_ref().unwrap().character_name.clone(),
          guild: character.last_update.as_ref().unwrap().character_guild.as_ref().and_then(|character_guild| self.get_guild(character_guild.guild_id)),
          last_updated: character.last_update.as_ref().unwrap().timestamp
        }
      })
      .collect();
    result.sort_by(|left: &CharacterSearchResult, right: &CharacterSearchResult| {
      if let Some(sorting) = filter.name.sorting {
        let ordering = left.name.cmp(&right.name);
        if ordering != Ordering::Equal {
          return negate_ordering(ordering,sorting);
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
        let ordering = left.server.value.cmp(&right.server.value);
        if ordering != Ordering::Equal {
          return negate_ordering(ordering,sorting);
        }
      }

      if let Some(sorting) = filter.gender.sorting {
        let ordering = left.gender.value.cmp(&right.gender.value);
        if ordering != Ordering::Equal {
          return negate_ordering(ordering,sorting);
        }
      }

      if let Some(sorting) = filter.race.sorting {
        let ordering = left.race.value.cmp(&right.race.value);
        if ordering != Ordering::Equal {
          return negate_ordering(ordering,sorting);
        }
      }

      if let Some(sorting) = filter.faction.sorting {
        let ordering = left.faction.value.cmp(&right.faction.value);
        if ordering != Ordering::Equal {
          return negate_ordering(ordering,sorting);
        }
      }

      if let Some(sorting) = filter.hero_class.sorting {
        let ordering = left.hero_class.value.cmp(&right.hero_class.value);
        if ordering != Ordering::Equal {
          return negate_ordering(ordering,sorting);
        }
      }

      if let Some(sorting) = filter.last_updated.sorting {
        let ordering = left.last_updated.cmp(&right.last_updated);
        if ordering != Ordering::Equal {
          return negate_ordering(ordering,sorting);
        }
      }

      return Ordering::Equal;
    });
    result
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