use crate::modules::instance::Instance;
use rocket::State;
use rocket_contrib::json::Json;
use crate::modules::instance::dto::{RankingResult, RankingCharacterMeta, InstanceFailure};
use crate::modules::armory::Armory;
use crate::modules::armory::tools::GetCharacter;
use std::collections::HashMap;
use crate::modules::data::Data;
use crate::modules::data::tools::{RetrieveNPC, RetrieveServer, RetrieveLocalization};
use crate::modules::data::guard::Language;

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

#[openapi]
#[get("/ranking/character/<character_id>")]
pub fn get_character_ranking(me: State<Instance>, data: State<Data>, armory: State<Armory>, language: Language, character_id: u32) -> Result<Json<Vec<(String, Option<RankingResult>, Option<RankingResult>, Option<RankingResult>)>>, InstanceFailure> {
    let character = armory.get_character(character_id).ok_or_else(|| InstanceFailure::InvalidInput)?;
    let server = data.get_server(character.server_id).unwrap();

    let rankings_dps = me.instance_rankings_dps.read().unwrap();
    let rankings_hps = me.instance_rankings_hps.read().unwrap();
    let rankings_tps = me.instance_rankings_tps.read().unwrap();

    // TODO: Extract as tool
    let mut result: HashMap<u32, (Option<RankingResult>, Option<RankingResult>, Option<RankingResult>)> = HashMap::new();
    rankings_dps.1.iter()
        .filter(|(_npc_id, char_rankings)| char_rankings.contains_key(&character_id))
        .for_each(|(npc_id, char_rankings)| {
            if !result.contains_key(npc_id) {
                result.insert(*npc_id, (None, None, None));
            }
            if let Some(rankings) = result.get_mut(npc_id) {
                rankings.0 = char_rankings.get(&character_id).map(helper_get_best_ranking);
            }
        });

    rankings_hps.1.iter()
        .filter(|(_npc_id, char_rankings)| char_rankings.contains_key(&character_id))
        .for_each(|(npc_id, char_rankings)| {
            if !result.contains_key(npc_id) {
                result.insert(*npc_id, (None, None, None));
            }
            if let Some(rankings) = result.get_mut(npc_id) {
                rankings.0 = char_rankings.get(&character_id).map(helper_get_best_ranking);
            }
        });

    rankings_tps.1.iter()
        .filter(|(_npc_id, char_rankings)| char_rankings.contains_key(&character_id))
        .for_each(|(npc_id, char_rankings)| {
            if !result.contains_key(npc_id) {
                result.insert(*npc_id, (None, None, None));
            }
            if let Some(rankings) = result.get_mut(npc_id) {
                rankings.0 = char_rankings.get(&character_id).map(helper_get_best_ranking);
            }
        });

    Ok(Json(result.into_iter()
        .map(|(npc_id, (r1, r2, r3))|
            (data.get_npc(server.expansion_id, npc_id).map(|npc|
                data.get_localization(language.0, npc.localization_id).unwrap().content)
                 .unwrap_or_else(|| String::from("Unknown")),
             r1, r2, r3))
        .collect()))
}

fn helper_get_best_ranking(ranking: &Vec<RankingResult>) -> RankingResult {
    ranking.iter().fold(RankingResult { attempt_id: 0, amount: 0, duration: 1 }, |best, ranking_result| {
        if (best.amount as f64 / best.duration as f64) < (ranking_result.amount as f64 / ranking_result.duration as f64) {
            return ranking_result.clone();
        }
        return best;
    })
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