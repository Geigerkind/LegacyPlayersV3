use crate::modules::armory::Armory;
use crate::modules::armory::tools::{GetCharacter, GetGuild};
use crate::modules::data::Data;
use crate::modules::data::tools::RetrieveServer;
use crate::modules::tooltip::domain_value::CharacterGuild;
use crate::modules::tooltip::dto::TooltipFailure;
use crate::modules::tooltip::material::CharacterTooltip;
use crate::modules::tooltip::Tooltip;

pub trait RetrieveCharacterTooltip {
  fn get_character(&self, data: &Data, armory: &Armory, character_id: u32) -> Result<CharacterTooltip, TooltipFailure>;
}

impl RetrieveCharacterTooltip for Tooltip {
  fn get_character(&self, data: &Data, armory: &Armory, character_id: u32) -> Result<CharacterTooltip, TooltipFailure> {
    let character_res = armory.get_character(character_id);
    if character_res.is_none() {
      return Err(TooltipFailure::InvalidInput);
    }
    let character = character_res.unwrap();

    if character.last_update.is_none() {
      return Err(TooltipFailure::CharacterHasNoInformation);
    }
    let character_history = character.last_update.unwrap();

    let guild = character_history.character_guild
      .and_then(|character_guild| armory.get_guild(character_guild.guild_id)
        .and_then(|guild| Some(CharacterGuild {
          name: guild.name.to_owned(),
          rank: character_guild.rank.to_owned(),
        })));

    Ok(CharacterTooltip {
      name: character_history.character_name.to_owned(),
      server: data.get_server(character.server_id).unwrap().name.to_owned(),
      guild,
    })
  }
}