use crate::modules::tooltip::material::CharacterTooltip;
use crate::dto::Failure;
use crate::modules::tooltip::Tooltip;
use crate::modules::armory::Armory;
use crate::modules::armory::tools::{GetCharacter, GetGuild};
use crate::modules::tooltip::domain_value::CharacterGuild;
use crate::modules::data::Data;
use crate::modules::data::tools::RetrieveServer;

pub trait RetrieveCharacterTooltip {
    fn get_character(&self, data: &Data, armory: &Armory, character_id: u32) -> Result<CharacterTooltip, Failure>;
}

impl RetrieveCharacterTooltip for Tooltip {
    fn get_character(&self, data: &Data, armory: &Armory, character_id: u32) -> Result<CharacterTooltip, Failure> {
        let character_res = armory.get_character(character_id);
        if character_res.is_none() {
            return Err(Failure::InvalidInput);
        }
        let character = character_res.unwrap();

        if character.last_update.is_none() {
            return Err(Failure::Unknown); // TODO: Better failure here
        }
        let character_history = character.last_update.unwrap();

        // TODO: Improve data representation here
        let guild_rank = character_history.guild_rank.to_owned();
        let guild = character_history.guild_id
            .and_then(|id| armory.get_guild(id)
                .and_then(|guild| Some(CharacterGuild {
                    name: guild.name.to_owned(),
                    rank: guild_rank.unwrap().to_owned()
                })));

        Ok(CharacterTooltip {
            name: character_history.character_name.to_owned(),
            server: data.get_server(character.server_id).unwrap().name.to_owned(),
            guild
        })
    }
}