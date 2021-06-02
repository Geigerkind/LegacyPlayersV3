use rocket::State;
use rocket_contrib::json::Json;

use crate::MainDb;
use crate::modules::account::guard::IsModerator;
use crate::modules::armory::Armory;
use crate::modules::instance::dto::{InstanceFailure, RankingCharacterMeta, RankingResult};
use crate::modules::instance::Instance;
use crate::modules::instance::tools::{create_ranking_export, UnrankAttempt};

#[openapi]
#[get("/ranking/dps")]
pub fn get_instance_ranking_dps(me: State<Instance>, armory: State<Armory>) -> Json<Vec<(u32, Vec<(u32, RankingCharacterMeta, Vec<RankingResult>)>)>> {
    let instance_metas = me.instance_metas.read().unwrap();
    let rankings = me.instance_rankings_dps.read().unwrap();
    Json(create_ranking_export(&instance_metas.1, &rankings.1, &armory))
}

#[openapi]
#[get("/ranking/hps")]
pub fn get_instance_ranking_hps(me: State<Instance>, armory: State<Armory>) -> Json<Vec<(u32, Vec<(u32, RankingCharacterMeta, Vec<RankingResult>)>)>> {
    let instance_metas = me.instance_metas.read().unwrap();
    let rankings = me.instance_rankings_hps.read().unwrap();
    Json(create_ranking_export(&instance_metas.1, &rankings.1, &armory))
}

#[openapi]
#[get("/ranking/tps")]
pub fn get_instance_ranking_tps(me: State<Instance>, armory: State<Armory>) -> Json<Vec<(u32, Vec<(u32, RankingCharacterMeta, Vec<RankingResult>)>)>> {
    let instance_metas = me.instance_metas.read().unwrap();
    let rankings = me.instance_rankings_tps.read().unwrap();
    Json(create_ranking_export(&instance_metas.1, &rankings.1, &armory))
}

#[openapi]
#[delete("/ranking/unrank", data = "<data>")]
pub fn unrank_attempt(mut db_main: MainDb, me: State<Instance>, data: Json<u32>, _auth: IsModerator) -> Result<(), InstanceFailure> {
    me.unrank_attempt(&mut *db_main, data.into_inner())
}