use crate::modules::armory::tools::GetCharacter;
use crate::modules::armory::Armory;
use crate::modules::data::tools::{RetrieveLocalization, RetrieveNPC, RetrieveServer};
use crate::modules::data::Data;
use crate::modules::instance::dto::{InstanceFailure, RankingCharacterMeta, RankingResult};
use crate::modules::instance::Instance;
use std::collections::HashMap;

pub trait ExportRanking {
    fn get_character_ranking(&self, data: &Data, armory: &Armory, language_id: u8, character_id: u32) -> Result<Vec<(String, Option<RankingResult>, Option<RankingResult>, Option<RankingResult>)>, InstanceFailure>;
}

impl ExportRanking for Instance {
    fn get_character_ranking(&self, data: &Data, armory: &Armory, language_id: u8, character_id: u32) -> Result<Vec<(String, Option<RankingResult>, Option<RankingResult>, Option<RankingResult>)>, InstanceFailure> {
        let character = armory.get_character(character_id).ok_or_else(|| InstanceFailure::InvalidInput)?;
        let server = data.get_server(character.server_id).unwrap();

        let rankings_dps = self.instance_rankings_dps.read().unwrap();
        let rankings_hps = self.instance_rankings_hps.read().unwrap();
        let rankings_tps = self.instance_rankings_tps.read().unwrap();

        // TODO: Extract as tool
        let mut result: HashMap<u32, (Option<RankingResult>, Option<RankingResult>, Option<RankingResult>)> = HashMap::new();
        rankings_dps.1.iter().filter(|(_npc_id, char_rankings)| char_rankings.contains_key(&character_id)).for_each(|(npc_id, char_rankings)| {
            let rankings = result.entry(*npc_id).or_insert((None, None, None));
            rankings.0 = char_rankings.get(&character_id).map(helper_get_best_ranking);
        });

        rankings_hps.1.iter().filter(|(_npc_id, char_rankings)| char_rankings.contains_key(&character_id)).for_each(|(npc_id, char_rankings)| {
            let rankings = result.entry(*npc_id).or_insert((None, None, None));
            rankings.1 = char_rankings.get(&character_id).map(helper_get_best_ranking);
        });

        rankings_tps.1.iter().filter(|(_npc_id, char_rankings)| char_rankings.contains_key(&character_id)).for_each(|(npc_id, char_rankings)| {
            let rankings = result.entry(*npc_id).or_insert((None, None, None));
            rankings.2 = char_rankings.get(&character_id).map(helper_get_best_ranking);
        });

        Ok(result
            .into_iter()
            .map(|(npc_id, (r1, r2, r3))| {
                (
                    data.get_npc(server.expansion_id, npc_id)
                        .map(|npc| data.get_localization(language_id, npc.localization_id).unwrap().content)
                        .unwrap_or_else(|| String::from("Unknown")),
                    r1,
                    r2,
                    r3,
                )
            })
            .collect())
    }
}

pub fn create_ranking_export(rankings: &HashMap<u32, HashMap<u32, Vec<RankingResult>>>, armory: &Armory) -> Vec<(u32, Vec<(u32, RankingCharacterMeta, Vec<RankingResult>)>)> {
    rankings
        .iter()
        .map(|(npc_id, char_rankings)| {
            (
                *npc_id,
                char_rankings
                    .iter()
                    .map(|(character_id, rankings)| {
                        (
                            *character_id,
                            armory
                                .get_character(*character_id)
                                .map(|character| RankingCharacterMeta {
                                    server_id: character.server_id,
                                    hero_class_id: character.last_update.as_ref().map(|last_update| last_update.character_info.hero_class_id).unwrap_or_else(|| 0),
                                    name: character.last_update.as_ref().map(|last_update| last_update.character_name.clone()).unwrap_or_else(|| String::from("Unknown")),
                                })
                                .unwrap(),
                            rankings.clone(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn helper_get_best_ranking(ranking: &Vec<RankingResult>) -> RankingResult {
    ranking.iter().fold(RankingResult { attempt_id: 0, amount: 0, duration: 1 }, |best, ranking_result| {
        if (best.amount as f64 / best.duration as f64) < (ranking_result.amount as f64 / ranking_result.duration as f64) {
            return ranking_result.clone();
        }
        best
    })
}
