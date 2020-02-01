use crate::modules::armory::dto::{CharacterViewerDto, ArmoryFailure, CharacterViewerGearDto, CharacterViewerGuildDto, CharacterViewerItemDto, CharacterViewerProfessionDto};
use crate::modules::armory::Armory;
use crate::modules::armory::tools::{GetCharacter, GetCharacterHistory, GetGuild};
use crate::modules::data::Data;
use crate::modules::data::tools::{RetrieveRace, RetrieveItem, RetrieveServer, RetrieveIcon, RetrieveTitle, RetrieveLocalization, RetrieveProfession};
use crate::dto::SelectOption;
use crate::modules::armory::domain_value::CharacterItem;

pub trait CharacterViewer {
  fn get_character_viewer_by_history_id(&self, data: &Data, language_id: u8, character_history_id: u32, character_id: u32) -> Result<CharacterViewerDto, ArmoryFailure>;
  fn get_character_viewer(&self, data: &Data, language_id: u8, character_id: u32) -> Result<CharacterViewerDto, ArmoryFailure>;
}

impl CharacterViewer for Armory {
  fn get_character_viewer_by_history_id(&self, data: &Data, language_id: u8, character_history_id: u32, character_id: u32) -> Result<CharacterViewerDto, ArmoryFailure> {
    let character = self.get_character(character_id);
    if character.is_none() || character.as_ref().unwrap().last_update.is_none() || character.as_ref().unwrap().history_moments.iter().find(|hm| hm.id == character_history_id).is_none() {
      return Err(ArmoryFailure::InvalidInput);
    }
    let character_res = character.unwrap();

    let character_history;
    if character_res.last_update.as_ref().unwrap().id == character_history_id {
      character_history = character_res.last_update.unwrap();
    } else {
      character_history = self.get_character_history(character_history_id).unwrap();
    }

    let race = data.get_race(character_history.character_info.race_id).unwrap();
    let server = data.get_server(character_res.server_id).unwrap();

    let mut profession_points_max: u16 = 300;
    if server.expansion_id == 2 {
      profession_points_max = 375;
    } else if server.expansion_id == 3 {
      profession_points_max = 450;
    }

    let profession1 = character_history.character_info.profession1
      .and_then(|profession_id| data.get_profession(profession_id).and_then(|profession| Some(CharacterViewerProfessionDto {
        icon: data.get_icon(profession.icon).unwrap().name,
        name: data.get_localization(language_id, profession.localization_id).unwrap().content,
        points: character_history.profession_skill_points1.unwrap(),
        point_max: profession_points_max
      })));
    let profession2  = character_history.character_info.profession2
      .and_then(|profession_id| data.get_profession(profession_id).and_then(|profession| Some(CharacterViewerProfessionDto {
        icon: data.get_icon(profession.icon).unwrap().name,
        name: data.get_localization(language_id, profession.localization_id).unwrap().content,
        points: character_history.profession_skill_points2.unwrap(),
        point_max: profession_points_max
      })));

    Ok(CharacterViewerDto {
      history_id: character_history_id,
      character_id,
      gender: character_history.character_info.gender,
      race_id: race.id,
      hero_class_id: character_history.character_info.hero_class_id,
      faction: race.faction,
      level: character_history.character_info.level,
      name: character_history.character_title.and_then(|title_id| data.get_title(title_id).and_then(|title|
        data.get_localization(language_id, title.localization_id)
          .and_then(|localization| Some(localization.content.replace("%s", &character_history.character_name)))))
        .or_else(|| Some(character_history.character_name.clone())).unwrap(),
      guild: character_history.character_guild.and_then(|guild| Some(CharacterViewerGuildDto {
        guild_id: guild.guild_id,
        name: self.get_guild(guild.guild_id).unwrap().name,
        rank: guild.rank
      })),
      history: character_res.history_moments.iter().map(|history_moment| SelectOption {
        value: history_moment.id,
        label_key: history_moment.timestamp.to_string()
      }).collect(),
      gear: CharacterViewerGearDto {
        gear_id: character_history.character_info.gear.id,
        head: character_history.character_info.gear.head.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        neck: character_history.character_info.gear.neck.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        shoulder: character_history.character_info.gear.shoulder.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        back: character_history.character_info.gear.back.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        chest: character_history.character_info.gear.chest.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        shirt: character_history.character_info.gear.shirt.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        tabard: character_history.character_info.gear.tabard.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        wrist: character_history.character_info.gear.wrist.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        main_hand: character_history.character_info.gear.main_hand.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        off_hand: character_history.character_info.gear.off_hand.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        ternary_hand: character_history.character_info.gear.ternary_hand.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        glove: character_history.character_info.gear.glove.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        belt: character_history.character_info.gear.belt.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        leg: character_history.character_info.gear.leg.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        boot: character_history.character_info.gear.boot.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        ring1: character_history.character_info.gear.ring1.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        ring2: character_history.character_info.gear.ring2.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        trinket1: character_history.character_info.gear.trinket1.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner))),
        trinket2: character_history.character_info.gear.trinket2.and_then(|inner| Some(character_item_to_character_item_viewer_dto(data, server.expansion_id, inner)))
      },
      profession1,
      profession2
    })
  }

  fn get_character_viewer(&self, data: &Data, language_id: u8, character_id: u32) -> Result<CharacterViewerDto, ArmoryFailure> {
    let character = self.get_character(character_id);
    if character.is_none() || character.as_ref().unwrap().last_update.is_none() {
      return Err(ArmoryFailure::InvalidInput);
    }
    return self.get_character_viewer_by_history_id(data, language_id, character.unwrap().last_update.unwrap().id, character_id);
  }
}

fn character_item_to_character_item_viewer_dto(data: &Data, expansion_id: u8, character_item: CharacterItem) -> CharacterViewerItemDto {
  let item = data.get_item(expansion_id, character_item.item_id).unwrap();
  CharacterViewerItemDto {
    item_id: character_item.item_id,
    quality: item.quality,
    icon: data.get_icon(item.icon).unwrap().name
  }
}