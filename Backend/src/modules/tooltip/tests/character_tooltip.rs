use crate::modules::tooltip::Tooltip;
use crate::modules::data::Data;
use crate::modules::armory::Armory;
use crate::modules::armory::dto::{CharacterInfoDto, CharacterGearDto, CharacterItemDto, CharacterDto, CharacterHistoryDto, GuildDto, CharacterGuildDto};
use crate::modules::armory::tools::SetCharacter;
use mysql_connection::tools::Execute;
use crate::modules::tooltip::tools::RetrieveCharacterTooltip;
use crate::modules::data::tools::RetrieveServer;

#[test]
fn character_tooltip() {
    let tooltip = Tooltip::default().init();
    let data = Data::default().init(None);
    let armory = Armory::default();

    let character_info_dto = CharacterInfoDto {
        gear: CharacterGearDto {
            head: None,
            neck: None,
            shoulder: None,
            back: None,
            chest: None,
            shirt: None,
            tabard: None,
            wrist: None,
            main_hand: Some(CharacterItemDto {
                item_id: 17,
                random_property_id: None,
                enchant_id: Some(266),
                gem_ids: vec![None, Some(23094), Some(23094), None],
            }),
            off_hand: None,
            ternary_hand: None,
            glove: None,
            belt: None,
            leg: None,
            boot: None,
            ring1: None,
            ring2: None,
            trinket1: None,
            trinket2: None,
        },
        hero_class_id: 2,
        level: 24,
        gender: false,
        profession1: Some(4),
        profession2: None,
        talent_specialization: None,
        race_id: 4,
    };
    let character_history_dto = CharacterHistoryDto {
        character_info: character_info_dto.to_owned(),
        character_name: "sdgsdfsd".to_string(),
        character_guild: Some(CharacterGuildDto {
          guild: GuildDto {
            server_uid: 324234234,
            name: "Testadsdfsfdsf".to_string()
          },
          rank: "Test123sdfsd".to_string()
        }),
    };
    let character_dto = CharacterDto {
        server_uid: 433356,
        character_history: Some(character_history_dto.to_owned()),
    };

    let server = data.get_server(3).unwrap();

    let character_res = armory.set_character(3, character_dto);
    assert!(character_res.is_ok());
    let character = character_res.unwrap();

    let tooltip_res = tooltip.get_character(&data, &armory, character.id);
    assert!(tooltip_res.is_ok());
    let tooltip = tooltip_res.unwrap();

    assert_eq!(tooltip.name, character_history_dto.character_name);
    assert_eq!(tooltip.server, server.name);
    assert!(tooltip.guild.is_some());
    assert_eq!(tooltip.guild.as_ref().unwrap().name, character_history_dto.character_guild.as_ref().unwrap().guild.name);
    assert_eq!(tooltip.guild.as_ref().unwrap().rank, character_history_dto.character_guild.as_ref().unwrap().rank);

    let character_history = character.last_update.unwrap();
    armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.main_hand.unwrap().id));
    armory.db_main.execute_wparams("DELETE FROM armory_gear WHERE id=:id", params!("id" => character_history.character_info.gear.id));
    armory.db_main.execute_wparams("DELETE FROM armory_character_info WHERE id=:id", params!("id" => character_history.character_info.id));
    armory.db_main.execute_wparams("DELETE FROM armory_character_history WHERE id=:id", params!("id" => character_history.id));
    armory.db_main.execute_wparams("DELETE FROM armory_character WHERE id=:id", params!("id" => character.id));
    armory.db_main.execute_wparams("DELETE FROM armory_guild WHERE id=:id", params!("id" => character_history.character_guild.unwrap().guild_id));
}