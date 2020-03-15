use crate::modules::armory::dto::{ArmoryFailure, GuildViewerDto, GuildViewerMemberDto};
use crate::modules::armory::tools::GetGuild;
use crate::modules::armory::Armory;
use crate::modules::data::tools::RetrieveRace;
use crate::modules::data::Data;

pub trait GuildViewer {
    fn get_guild_view(
        &self,
        data: &Data,
        language_id: u8,
        guild_id: u32,
    ) -> Result<GuildViewerDto, ArmoryFailure>;
}

impl GuildViewer for Armory {
    fn get_guild_view(
        &self,
        data: &Data,
        _language_id: u8,
        guild_id: u32,
    ) -> Result<GuildViewerDto, ArmoryFailure> {
        let guild = self.get_guild(guild_id);
        if guild.is_none() {
            return Err(ArmoryFailure::InvalidInput);
        }
        let guild = guild.unwrap();

        let characters = self.characters.read().unwrap();
        let member = characters
            .iter()
            .filter(|(_, character)| character.last_update.is_some())
            .filter(|(_, character)| {
                character
                    .last_update
                    .as_ref()
                    .unwrap()
                    .character_guild
                    .is_some()
            })
            .filter(|(_, character)| {
                character
                    .last_update
                    .as_ref()
                    .unwrap()
                    .character_guild
                    .as_ref()
                    .unwrap()
                    .guild_id
                    == guild_id
            })
            .map(|(character_id, character)| {
                let last_update = character.last_update.as_ref().unwrap();
                let race = data.get_race(last_update.character_info.race_id).unwrap();

                GuildViewerMemberDto {
                    character_id: *character_id,
                    character_name: last_update.character_name.clone(),
                    faction: race.faction,
                    race_id: race.id,
                    hero_class_id: last_update.character_info.hero_class_id,
                    rank_index: last_update.character_guild.as_ref().unwrap().rank.index,
                    last_seen: last_update.timestamp,
                }
            })
            .collect();

        Ok(GuildViewerDto {
            guild_id,
            guild_name: guild.name,
            ranks: guild.ranks,
            member,
        })
    }
}
