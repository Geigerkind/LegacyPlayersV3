use std::collections::HashMap;

use crate::modules::armory::Armory;
use crate::modules::armory::tools::GetCharacter;
use crate::modules::instance::domain_value::{InstanceMeta, PrivacyType};
use crate::modules::instance::dto::{RankingCharacterMeta, RankingResult};

pub fn create_ranking_export(instance_metas: &HashMap<u32, InstanceMeta>, rankings: &HashMap<u32, HashMap<u32, Vec<RankingResult>>>, armory: &Armory) -> Vec<(u32, Vec<(u32, RankingCharacterMeta, Vec<RankingResult>)>)> {
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
                            rankings.iter().filter_map(|rr| {
                                let instance_meta = instance_metas.get(&rr.instance_meta_id)?;
                                if instance_meta.privacy_type == PrivacyType::Public {
                                    return Some(rr);
                                }
                                None
                            }).cloned().collect(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn helper_get_best_ranking(ranking: Vec<RankingResult>) -> RankingResult {
    ranking.iter().fold(
        RankingResult {
            instance_meta_id: 0,
            attempt_id: 0,
            amount: 0,
            duration: 1,
            difficulty_id: 0,
            character_spec: 0,
            season_index: 0,
        },
        |best, ranking_result| {
            if (best.amount as f64 / best.duration as f64) < (ranking_result.amount as f64 / ranking_result.duration as f64) {
                return ranking_result.clone();
            }
            best
        },
    )
}
