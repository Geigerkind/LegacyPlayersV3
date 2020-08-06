use crate::modules::armory::Armory;
use crate::modules::data::guard::Language;
use crate::modules::data::Data;
use crate::modules::instance::dto::{InstanceFailure, RankingCharacterMeta, RankingResult};
use crate::modules::instance::tools::{create_ranking_export, ExportRanking};
use crate::modules::instance::Instance;
use rocket::State;
use rocket_contrib::json::Json;

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
    me.get_character_ranking(&data, &armory, language.0, character_id).map(Json)
}
