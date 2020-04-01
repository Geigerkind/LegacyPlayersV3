#![allow(clippy::module_inception)]
#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro, option_result_contains, vec_remove_item, test)]
extern crate language;
extern crate mail;
#[macro_use]
extern crate mysql_connection;
extern crate okapi;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate str_util;
extern crate time_util;
extern crate validator;
#[macro_use]
extern crate lazy_static;
extern crate dotenv;
extern crate regex;
extern crate byteorder;

use dotenv::dotenv;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig, UrlObject};
use rocket_prometheus::PrometheusMetrics;

use crate::modules::{account, armory, data, tooltip, live_data_processor};

mod dto;
mod modules;
mod util;

fn main() {
    dotenv().ok();

    let account = account::Account::default().init();
    let data = data::Data::default().init(None);
    let armory = armory::Armory::default().init();
    let tooltip = tooltip::Tooltip::default().init();
    let live_data_processor = live_data_processor::LiveDataProcessor::default().init();

    let prometheus = PrometheusMetrics::new();
    let mut igniter = rocket::ignite();
    igniter = igniter.manage(account);
    igniter = igniter.manage(data);
    igniter = igniter.manage(armory);
    igniter = igniter.manage(tooltip);
    igniter = igniter.manage(live_data_processor);

    igniter = igniter.attach(prometheus.clone());
    igniter = igniter.mount("/metrics", prometheus);
    igniter = igniter.mount(
        "/API/",
        make_swagger_ui(&SwaggerUIConfig {
            url: None,
            urls: Some(vec![
                UrlObject {
                    name: "Account".to_string(),
                    url: "/API/account/openapi.json".to_string(),
                },
                UrlObject {
                    name: "Data".to_string(),
                    url: "/API/data/openapi.json".to_string(),
                },
                UrlObject {
                    name: "Armory".to_string(),
                    url: "/API/armory/openapi.json".to_string(),
                },
                UrlObject {
                    name: "Tooltip".to_string(),
                    url: "/API/tooltip/openapi.json".to_string(),
                },
                UrlObject {
                    name: "Live Data Processor".to_string(),
                    url: "/API/live_data_processor/openapi.json".to_string(),
                },
            ]),
        }),
    );
    igniter = igniter.mount(
        "/API/account/",
        routes_with_openapi![
            account::transfer::login::login,
            account::transfer::token::create_token,
            account::transfer::token::get_tokens,
            account::transfer::token::delete_token,
            account::transfer::token::prolong_token,
            account::transfer::delete::request,
            account::transfer::delete::confirm,
            account::transfer::create::create,
            account::transfer::create::confirm,
            account::transfer::create::resend_confirm,
            account::transfer::get::get_account_information,
            account::transfer::forgot::receive_confirmation,
            account::transfer::forgot::send_confirmation,
            account::transfer::update::request_mail,
            account::transfer::update::confirm_mail,
            account::transfer::update::password,
            account::transfer::update::nickname
        ],
    );

    igniter = igniter.mount(
        "/API/data/",
        routes_with_openapi![
            data::transfer::expansion::get_expansion,
            data::transfer::expansion::get_all_expansions,
            data::transfer::language::get_language,
            data::transfer::language::get_all_languages,
            data::transfer::language::get_language_by_short_code,
            data::transfer::localization::get_localization,
            data::transfer::race::get_race,
            data::transfer::race::get_all_races,
            data::transfer::race::get_race_localized,
            data::transfer::race::get_all_races_localized,
            data::transfer::profession::get_profession,
            data::transfer::profession::get_all_professions,
            data::transfer::server::get_server,
            data::transfer::server::get_all_servers,
            data::transfer::hero_class::get_hero_class,
            data::transfer::hero_class::get_all_hero_classes,
            data::transfer::hero_class::get_hero_class_localized,
            data::transfer::hero_class::get_all_hero_classes_localized,
            data::transfer::spell::get_spell,
            data::transfer::dispel_type::get_dispel_type,
            data::transfer::dispel_type::get_all_dispel_types,
            data::transfer::power_type::get_power_type,
            data::transfer::power_type::get_all_power_types,
            data::transfer::stat_type::get_stat_type,
            data::transfer::stat_type::get_all_stat_types,
            data::transfer::spell_effect::get_spell_effects,
            data::transfer::npc::get_npc,
            data::transfer::icon::get_icon,
            data::transfer::item::get_item,
            data::transfer::gem::get_gem,
            data::transfer::enchant::get_enchant,
            data::transfer::item_bonding::get_item_bonding,
            data::transfer::item_bonding::get_all_item_bondings,
            data::transfer::item_class::get_item_class,
            data::transfer::item_class::get_all_item_classes,
            data::transfer::item_damage::get_item_damage,
            data::transfer::item_damage_type::get_item_damage_type,
            data::transfer::item_damage_type::get_all_item_damage_types,
            data::transfer::item_effect::get_item_effect,
            data::transfer::item_inventory_type::get_item_inventory_type,
            data::transfer::item_inventory_type::get_all_item_inventory_types,
            data::transfer::item_quality::get_item_quality,
            data::transfer::item_quality::get_all_item_qualities,
            data::transfer::item_random_property::get_item_random_property,
            data::transfer::item_sheath::get_item_sheath,
            data::transfer::item_sheath::get_all_item_sheaths,
            data::transfer::item_socket::get_item_socket,
            data::transfer::item_stat::get_item_stats,
            data::transfer::itemset_name::get_itemset_name,
            data::transfer::itemset_name::get_itemset_item_ids,
            data::transfer::itemset_effect::get_itemset_effects,
            data::transfer::title::get_title,
            data::transfer::title::get_all_titles,
            data::transfer::item_random_property_points::get_item_random_property_points,
        ],
    );

    igniter = igniter.mount(
        "/API/armory/",
        routes_with_openapi![
            armory::transfer::character::set_character,
            armory::transfer::character::get_character,
            armory::transfer::character::get_character_by_uid,
            armory::transfer::character::delete_character,
            armory::transfer::character::delete_character_by_uid,
            armory::transfer::character::get_characters_by_name,
            armory::transfer::guild::get_guild,
            armory::transfer::guild::get_guilds_by_name,
            armory::transfer::guild::delete_guild,
            armory::transfer::guild::delete_guild_by_uid,
            armory::transfer::guild::create_guild,
            armory::transfer::guild::update_guild_name,
            armory::transfer::character_history::set_character_history,
            armory::transfer::character_history::get_character_history,
            armory::transfer::character_history::delete_character_history,
            armory::transfer::character_search::get_character_search_result,
            armory::transfer::character_viewer::get_character_viewer,
            armory::transfer::character_viewer::get_character_viewer_by_history,
            armory::transfer::guild_viewer::get_guild_view,
            armory::transfer::instance_reset::set_instance_resets
        ],
    );

    igniter = igniter.mount(
        "/API/tooltip/",
        routes_with_openapi![
            tooltip::transfer::item_tooltip::get_item,
            tooltip::transfer::item_tooltip::get_character_item,
            tooltip::transfer::spell_tooltip::get_spell,
            tooltip::transfer::character_tooltip::get_character,
            tooltip::transfer::guild_tooltip::get_guild,
        ],
    );

    igniter = igniter.mount(
        "/API/live_data_processor",
        routes_with_openapi![
            live_data_processor::transfer::package::get_package
        ]
    );

    igniter.launch();
}
