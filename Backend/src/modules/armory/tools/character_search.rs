use std::cmp::Ordering;

use crate::{
    {rpll_table_sort},
    dto::SearchResult,
    modules::{
        armory::{
            dto::{CharacterSearchCharacterDto, CharacterSearchFilter, CharacterSearchGuildDto, CharacterSearchResult},
            tools::GetGuild,
            Armory,
        },
        data::{tools::RetrieveRace, Data},
    },
};
use crate::util::ordering::NegateOrdExt;

pub trait PerformCharacterSearch {
    fn get_character_search_result(&self, data: &Data, filter: CharacterSearchFilter) -> SearchResult<CharacterSearchResult>;
}

impl PerformCharacterSearch for Armory {
    // This implementation is far away from being optimal.
    fn get_character_search_result(&self, data: &Data, filter: CharacterSearchFilter) -> SearchResult<CharacterSearchResult> {
        let mut filter = filter;
        let characters = self.characters.read().unwrap();
        if filter.name.filter.is_some() {
            *filter.name.filter.as_mut().unwrap() = filter.name.filter.as_ref().unwrap().to_lowercase();
        }
        if filter.guild.filter.is_some() {
            *filter.guild.filter.as_mut().unwrap() = filter.guild.filter.as_ref().unwrap().to_lowercase();
        }
        let mut result: Vec<CharacterSearchResult> = characters
            .iter()
            .filter(|(_, character)| character.last_update.is_some())
            .filter(|(_, character)| filter.server.filter.is_none() || filter.server.filter.contains(&character.server_id))
            .filter(|(_, character)| filter.name.filter.is_none() || character.last_update.as_ref().unwrap().character_name.to_lowercase().contains(filter.name.filter.as_ref().unwrap()))
            .filter(|(_, character)| filter.hero_class.filter.is_none() || filter.hero_class.filter.contains(&character.last_update.as_ref().unwrap().character_info.hero_class_id))
            .filter(|(_, character)| {
                if filter.last_updated.filter.is_none() {
                    return true;
                }
                let filter_timestamp = *filter.last_updated.filter.as_ref().unwrap();
                let current_timestamp = character.last_update.as_ref().unwrap().timestamp;
                current_timestamp >= filter_timestamp && current_timestamp <= filter_timestamp + 24 * 60 * 60
            })
            // This is very expensive
            .filter(|(_, character)| {
                if filter.guild.filter.is_none() {
                    return true;
                }
                if let Some(character_guild) = character.last_update.as_ref().unwrap().character_guild.as_ref() {
                    if let Some(guild) = self.get_guild(character_guild.guild_id) {
                        return guild.name.to_lowercase().contains(filter.guild.filter.as_ref().unwrap());
                    }
                }
                false
            })
            .map(|(_, character)| {
                let race_id = character.last_update.as_ref().unwrap().character_info.race_id;
                CharacterSearchResult {
                    faction: data.get_race(race_id).unwrap().faction,
                    guild: character
                        .last_update.as_ref().unwrap().character_guild.as_ref()
                        .and_then(|character_guild| self.get_guild(character_guild.guild_id).map(|guild| CharacterSearchGuildDto { guild_id: guild.id, name: guild.name })),
                    character: CharacterSearchCharacterDto {
                        character_id: character.id,
                        name: character.last_update.as_ref().unwrap().character_name.clone(),
                        hero_class_id: character.last_update.as_ref().unwrap().character_info.hero_class_id,
                        server_id: character.server_id,
                    },
                    timestamp: character.last_update.as_ref().unwrap().timestamp,
                }
            })
            .collect();
        let num_characters = result.len();

        result.sort_by(|left, right| rpll_table_sort! {
            (filter.hero_class, Some(&left.character.hero_class_id), Some(&right.character.hero_class_id)),
            (filter.name, Some(&left.character.name), Some(&right.character.name)),
            (filter.guild, left.guild.as_ref().map(|gld| &gld.name), right.guild.as_ref().map(|gld| &gld.name)),
            (filter.server, Some(&left.character.server_id), Some(&right.character.server_id)),
            (filter.last_updated, Some(&left.timestamp), Some(&right.timestamp))
        });

        SearchResult {
            result: result.iter().skip((filter.page * 10) as usize).take(10).map(|cs| cs.to_owned()).collect::<Vec<CharacterSearchResult>>(),
            num_items: num_characters,
        }
    }
}
