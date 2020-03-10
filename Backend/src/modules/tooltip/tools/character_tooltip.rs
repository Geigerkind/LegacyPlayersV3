use crate::modules::armory::domain_value::CharacterItem;
use crate::modules::armory::tools::{GetCharacter, GetGuild};
use crate::modules::armory::Armory;
use crate::modules::data::tools::{
    RetrieveItem, RetrieveLocalization, RetrieveRace, RetrieveServer,
};
use crate::modules::data::Data;
use crate::modules::tooltip::domain_value::{CharacterGuild, CharacterTooltipItem};
use crate::modules::tooltip::dto::TooltipFailure;
use crate::modules::tooltip::material::CharacterTooltip;
use crate::modules::tooltip::Tooltip;

pub trait RetrieveCharacterTooltip {
    fn get_character(
        &self,
        data: &Data,
        armory: &Armory,
        language_id: u8,
        character_id: u32,
    ) -> Result<CharacterTooltip, TooltipFailure>;
}

impl RetrieveCharacterTooltip for Tooltip {
    fn get_character(
        &self,
        data: &Data,
        armory: &Armory,
        language_id: u8,
        character_id: u32,
    ) -> Result<CharacterTooltip, TooltipFailure> {
        let character_res = armory.get_character(character_id);
        if character_res.is_none() {
            return Err(TooltipFailure::InvalidInput);
        }
        let character = character_res.unwrap();

        if character.last_update.is_none() {
            return Err(TooltipFailure::CharacterHasNoInformation);
        }
        let character_history = character.last_update.unwrap();

        let guild = character_history
            .character_guild
            .and_then(|character_guild| {
                armory
                    .get_guild(character_guild.guild_id)
                    .and_then(|guild| {
                        Some(CharacterGuild {
                            name: guild.name.to_owned(),
                            rank: character_guild.rank.name.to_owned(),
                        })
                    })
            });

        let server = data.get_server(character.server_id).unwrap();
        let race = data
            .get_race(character_history.character_info.race_id)
            .unwrap();

        let mut items: Vec<CharacterTooltipItem> = Vec::new();
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.head,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.neck,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.shoulder,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.back,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.chest,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.tabard,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.shirt,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.wrist,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.glove,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.belt,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.leg,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.boot,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.ring1,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.ring2,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.trinket1,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.trinket2,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.main_hand,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.off_hand,
        );
        add_item(
            &data,
            server.expansion_id,
            language_id,
            &mut items,
            &character_history.character_info.gear.ternary_hand,
        );

        Ok(CharacterTooltip {
            name: character_history.character_name.to_owned(),
            server: server.name,
            guild,
            faction: race.faction,
            hero_class_id: character_history.character_info.hero_class_id,
            expansion_id: server.expansion_id,
            race_id: race.id,
            gender: character_history.character_info.gender,
            level: character_history.character_info.level,
            items,
        })
    }
}

fn add_item(
    data: &Data,
    expansion_id: u8,
    language_id: u8,
    vec: &mut Vec<CharacterTooltipItem>,
    item: &Option<CharacterItem>,
) {
    if let Some(character_item) = item {
        let item_template = data.get_item(expansion_id, character_item.item_id).unwrap();
        vec.push(CharacterTooltipItem {
            name: data
                .get_localization(language_id, item_template.localization_id)
                .unwrap()
                .content,
            quality: item_template.quality,
        });
    }
}
