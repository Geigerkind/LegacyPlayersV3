use crate::modules::instance::Instance;
use rocket::State;
use rocket_contrib::json::Json;
use crate::modules::instance::dto::{RankingResult, RankingCharacterMeta};
use crate::modules::armory::Armory;
use crate::modules::armory::tools::GetCharacter;
use std::collections::HashMap;

#[openapi]
#[get("/ranking/dps")]
pub fn get_instance_ranking_dps(me: State<Instance>, armory: State<Armory>) -> Json<Vec<(u32, Vec<(u32, RankingCharacterMeta, Vec<RankingResult>)>)>> {
    let rankings = me.instance_rankings_dps.read().unwrap();
    Json(create_ranking_export(&rankings.1, &armory))
}

#[openapi]
#[get("/ranking/hps")]
pub fn get_instance_ranking_hps(me: State<Instance>, armory: State<Armory>) -> Json<Vec<(u32, Vec<(u32, RankingCharacterMeta, Vec<RankingResult>)>)>> {
    let rankings = me.instance_rankings_hps.read().unwrap();
    Json(create_ranking_export(&rankings.1, &armory))
}

#[openapi]
#[get("/ranking/tps")]
pub fn get_instance_ranking_tps(me: State<Instance>, armory: State<Armory>) -> Json<Vec<(u32, Vec<(u32, RankingCharacterMeta, Vec<RankingResult>)>)>> {
    let rankings = me.instance_rankings_tps.read().unwrap();
    Json(create_ranking_export(&rankings.1, &armory))
}

// TODO: Create tool
fn create_ranking_export(rankings: &HashMap<u32, HashMap<u32, Vec<RankingResult>>>, armory: &Armory) -> Vec<(u32, Vec<(u32, RankingCharacterMeta, Vec<RankingResult>)>)> {
    rankings.iter()
        .map(|(npc_id, char_rankings)|
            (*npc_id, char_rankings.iter()
                .map(|(character_id, rankings)|
                    (*character_id,
                     armory.get_character(*character_id).map(|character| RankingCharacterMeta {
                         server_id: character.server_id,
                         hero_class_id: character.last_update.as_ref().map(|last_update| last_update.character_info.hero_class_id).unwrap_or_else(|| 0),
                         name: character.last_update.as_ref().map(|last_update| last_update.character_name.clone()).unwrap_or_else(|| String::from("Unknown")),
                     }).unwrap(),
                     rankings.clone()))
                .collect()))
        .collect()
}