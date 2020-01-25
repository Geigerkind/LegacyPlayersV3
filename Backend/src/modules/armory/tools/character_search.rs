use crate::modules::armory::dto::{CharacterSearchResult, CharacterSearchFilter};
use crate::modules::data::Data;
use crate::modules::armory::Armory;
use crate::dto::SelectOption;
use crate::modules::data::tools::{RetrieveLocalization, RetrieveRace, RetrieveServer, RetrieveHeroClass};

pub trait PerformCharacterSearch {
  fn get_character_search_result(&self, data: &Data, language_id: u8, filter: CharacterSearchFilter) -> Vec<CharacterSearchResult>;
}

impl PerformCharacterSearch for Armory {
  fn get_character_search_result(&self, data: &Data, language_id: u8, filter: CharacterSearchFilter) -> Vec<CharacterSearchResult> {
    let characters = self.characters.read().unwrap();
    let mut result = characters.iter()
      .filter(|(_, character)| character.last_update.is_some())
      .filter(|(_, character)| filter.server.filter.is_none() || filter.server.filter.contains(&character.server_id))
      .filter(|(_, character)| filter.name.filter.is_none() || character.last_update.as_ref().unwrap().character_name.contains(filter.name.filter.as_ref().unwrap()))
      .filter(|(_, character)| filter.race.filter.is_none() || filter.race.filter.contains(&character.last_update.as_ref().unwrap().character_info.race_id))
      .filter(|(_, character)| filter.gender.filter.is_none() || ((*filter.gender.filter.as_ref().unwrap()) != 0) == character.last_update.as_ref().unwrap().character_info.gender)
      .filter(|(_, character)| filter.hero_class.filter.is_none() || filter.hero_class.filter.contains(&character.last_update.as_ref().unwrap().character_info.hero_class_id))
      // TODO
      .filter(|(_, character)| filter.last_updated.filter.is_none() || filter.last_updated.filter.contains(&character.last_update.as_ref().unwrap().timestamp))
      // TODO: Faction
      .take(10)
      .map(|(_, character)| {
        let gender = character.last_update.as_ref().unwrap().character_info.gender as u8;
        let race_id = character.last_update.as_ref().unwrap().character_info.race_id;
        let hero_class_id = character.last_update.as_ref().unwrap().character_info.hero_class_id;
        CharacterSearchResult {
          gender: SelectOption { label_key: "General.gender_".to_owned() + &gender.to_string(), value: gender },
          race: SelectOption { label_key: data.get_localization(language_id, data.get_race(race_id).unwrap().localization_id).unwrap().content.to_owned(), value: race_id },
          faction: SelectOption { label_key: "General.faction_0".to_owned(), value: 0 },
          server: SelectOption { label_key: data.get_server(character.server_id).unwrap().name.to_owned(), value: character.server_id },
          hero_class: SelectOption { label_key: data.get_localization(language_id, data.get_hero_class(hero_class_id).unwrap().localization_id).unwrap().content.to_owned(), value: hero_class_id },
          name: character.last_update.as_ref().unwrap().character_name.clone(),
          last_updated: character.last_update.as_ref().unwrap().timestamp
        }
      })
      .collect();
    //result.sort_by();
    result
  }
}