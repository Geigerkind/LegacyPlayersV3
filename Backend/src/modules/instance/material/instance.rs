use crate::material::Cachable;
use crate::modules::armory::tools::GetArenaTeam;
use crate::modules::armory::Armory;
use crate::modules::instance::domain_value::{InstanceMeta, MetaType};
use crate::modules::instance::dto::{InstanceViewerAttempt, RankingResult};
use crate::params;
use crate::util::database::Select;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Instance {
    pub instance_metas: Arc<RwLock<HashMap<u32, InstanceMeta>>>,
    pub instance_exports: Arc<RwLock<HashMap<(u32, u8), Cachable<Vec<(u32, String)>>>>>,
    pub instance_attempts: Arc<RwLock<HashMap<u32, Cachable<Vec<InstanceViewerAttempt>>>>>,
    // encounter_id => character_id => Vec<Ranking>
    pub instance_rankings_dps: Arc<RwLock<(u32, HashMap<u32, HashMap<u32, Vec<RankingResult>>>)>>,
    pub instance_rankings_hps: Arc<RwLock<(u32, HashMap<u32, HashMap<u32, Vec<RankingResult>>>)>>,
    pub instance_rankings_tps: Arc<RwLock<(u32, HashMap<u32, HashMap<u32, Vec<RankingResult>>>)>>,
}

impl Default for Instance {
    fn default() -> Self {
        Instance {
            instance_metas: Arc::new(RwLock::new(HashMap::new())),
            instance_exports: Arc::new(RwLock::new(HashMap::new())),
            instance_attempts: Arc::new(RwLock::new(HashMap::new())),
            instance_rankings_dps: Arc::new(RwLock::new((0, HashMap::new()))),
            instance_rankings_hps: Arc::new(RwLock::new((0, HashMap::new()))),
            instance_rankings_tps: Arc::new(RwLock::new((0, HashMap::new()))),
        }
    }
}

impl Instance {
    pub fn init(self, mut db_main: (impl Select + Send + 'static), armory: &Armory) -> Self {
        update_instance_metas(Arc::clone(&self.instance_metas), &mut db_main, &armory);
        update_instance_rankings_dps(Arc::clone(&self.instance_rankings_dps), &mut db_main);
        update_instance_rankings_hps(Arc::clone(&self.instance_rankings_hps), &mut db_main);
        update_instance_rankings_tps(Arc::clone(&self.instance_rankings_tps), &mut db_main);

        let instance_metas_arc_clone = Arc::clone(&self.instance_metas);
        let instance_exports_arc_clone = Arc::clone(&self.instance_exports);
        let instance_attempts_arc_clone = Arc::clone(&self.instance_attempts);
        let instance_rankings_dps_arc_clone = Arc::clone(&self.instance_rankings_dps);
        let instance_rankings_hps_arc_clone = Arc::clone(&self.instance_rankings_hps);
        let instance_rankings_tps_arc_clone = Arc::clone(&self.instance_rankings_tps);
        std::thread::spawn(move || {
            let armory = Armory::default();
            loop {
                evict_attempts_cache(Arc::clone(&instance_attempts_arc_clone));
                evict_export_cache(Arc::clone(&instance_exports_arc_clone));
                update_instance_metas(Arc::clone(&instance_metas_arc_clone), &mut db_main, &armory);
                update_instance_rankings_dps(Arc::clone(&instance_rankings_dps_arc_clone), &mut db_main);
                update_instance_rankings_hps(Arc::clone(&instance_rankings_hps_arc_clone), &mut db_main);
                update_instance_rankings_tps(Arc::clone(&instance_rankings_tps_arc_clone), &mut db_main);
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        });
        self
    }

    pub fn update_instance_meta(&self, db_main: &mut impl Select, armory: &Armory) {
        update_instance_metas(Arc::clone(&self.instance_metas), db_main, armory);
    }
}

fn update_instance_rankings_dps(instance_rankings_dps: Arc<RwLock<(u32, HashMap<u32, HashMap<u32, Vec<RankingResult>>>)>>, db_main: &mut impl Select) {
    let mut rankings_dps = instance_rankings_dps.write().unwrap();
    db_main
        .select_wparams(
            "SELECT A.id, A.character_id, B.encounter_id, A.attempt_id, A.damage, (B.end_ts - B.start_ts) as duration, B.instance_meta_id FROM instance_ranking_damage A JOIN instance_attempt B ON A.attempt_id = B.id WHERE A.id > :last_queried_id \
             ORDER BY A.id",
            |mut row| {
                let id: u32 = row.take(0).unwrap();
                let character_id: u32 = row.take(1).unwrap();
                let encounter_id: u32 = row.take(2).unwrap();
                (
                    id,
                    character_id,
                    encounter_id,
                    RankingResult {
                        attempt_id: row.take(3).unwrap(),
                        amount: row.take(4).unwrap(),
                        duration: row.take(5).unwrap(),
                        instance_meta_id: row.take(6).unwrap(),
                    },
                )
            },
            params!("last_queried_id" => rankings_dps.0),
        )
        .into_iter()
        .for_each(|(id, character_id, encounter_id, ranking)| {
            rankings_dps.0 = id;
            let characters_rankings = rankings_dps.1.entry(encounter_id).or_insert_with(HashMap::new);
            let rankings = characters_rankings.entry(character_id).or_insert_with(|| Vec::with_capacity(1));
            rankings.push(ranking);
        });
}

fn update_instance_rankings_hps(instance_rankings_hps: Arc<RwLock<(u32, HashMap<u32, HashMap<u32, Vec<RankingResult>>>)>>, db_main: &mut impl Select) {
    let mut rankings_hps = instance_rankings_hps.write().unwrap();
    db_main
        .select_wparams(
            "SELECT A.id, A.character_id, B.encounter_id, A.attempt_id, A.heal, (B.end_ts - B.start_ts) as duration, B.instance_meta_id FROM instance_ranking_heal A JOIN instance_attempt B ON A.attempt_id = B.id WHERE A.id > :last_queried_id ORDER \
             BY A.id",
            |mut row| {
                let id: u32 = row.take(0).unwrap();
                let character_id: u32 = row.take(1).unwrap();
                let encounter_id: u32 = row.take(2).unwrap();
                (
                    id,
                    character_id,
                    encounter_id,
                    RankingResult {
                        attempt_id: row.take(3).unwrap(),
                        amount: row.take(4).unwrap(),
                        duration: row.take(5).unwrap(),
                        instance_meta_id: row.take(6).unwrap(),
                    },
                )
            },
            params!("last_queried_id" => rankings_hps.0),
        )
        .into_iter()
        .for_each(|(id, character_id, encounter_id, ranking)| {
            rankings_hps.0 = id;
            let characters_rankings = rankings_hps.1.entry(encounter_id).or_insert_with(HashMap::new);
            let rankings = characters_rankings.entry(character_id).or_insert_with(|| Vec::with_capacity(1));
            rankings.push(ranking);
        });
}

fn update_instance_rankings_tps(instance_rankings_tps: Arc<RwLock<(u32, HashMap<u32, HashMap<u32, Vec<RankingResult>>>)>>, db_main: &mut impl Select) {
    let mut rankings_tps = instance_rankings_tps.write().unwrap();
    db_main
        .select_wparams(
            "SELECT A.id, A.character_id, B.encounter_id, A.attempt_id, A.threat, (B.end_ts - B.start_ts) as duration, B.instance_meta_id FROM instance_ranking_threat A JOIN instance_attempt B ON A.attempt_id = B.id WHERE A.id > :last_queried_id \
             ORDER BY A.id",
            |mut row| {
                let id: u32 = row.take(0).unwrap();
                let character_id: u32 = row.take(1).unwrap();
                let encounter_id: u32 = row.take(2).unwrap();
                (
                    id,
                    character_id,
                    encounter_id,
                    RankingResult {
                        attempt_id: row.take(3).unwrap(),
                        amount: row.take(4).unwrap(),
                        duration: row.take(5).unwrap(),
                        instance_meta_id: row.take(6).unwrap(),
                    },
                )
            },
            params!("last_queried_id" => rankings_tps.0),
        )
        .into_iter()
        .for_each(|(id, character_id, encounter_id, ranking)| {
            rankings_tps.0 = id;
            let characters_rankings = rankings_tps.1.entry(encounter_id).or_insert_with(HashMap::new);
            let rankings = characters_rankings.entry(character_id).or_insert_with(|| Vec::with_capacity(1));
            rankings.push(ranking);
        });
}

fn evict_attempts_cache(instance_attempts: Arc<RwLock<HashMap<u32, Cachable<Vec<InstanceViewerAttempt>>>>>) {
    let now = time_util::now();
    let mut instance_attempts = instance_attempts.write().unwrap();
    for instance_meta_id in instance_attempts
        .iter()
        .filter(|(_, cachable)| cachable.get_last_access() + 3600 < now)
        .map(|(instance_meta_id, _)| *instance_meta_id)
        .collect::<Vec<u32>>()
    {
        instance_attempts.remove(&instance_meta_id);
    }
}

fn evict_export_cache(instance_exports: Arc<RwLock<HashMap<(u32, u8), Cachable<Vec<(u32, String)>>>>>) {
    let now = time_util::now();
    let mut instance_exports = instance_exports.write().unwrap();
    for instance_meta_id in instance_exports
        .iter()
        .filter(|(_, cachable)| cachable.get_last_access() + 3600 < now)
        .map(|(instance_meta_id, _)| *instance_meta_id)
        .collect::<Vec<(u32, u8)>>()
    {
        instance_exports.remove(&instance_meta_id);
    }
}

fn update_instance_metas(instance_metas: Arc<RwLock<HashMap<u32, InstanceMeta>>>, db_main: &mut impl Select, armory: &Armory) {
    let mut instance_metas = instance_metas.write().unwrap();
    instance_metas.clear();

    // Raids
    db_main
        .select(
            "SELECT A.id, A.server_id, A.start_ts, A.end_ts, A.expired, A.map_id, B.map_difficulty, A.uploaded_user FROM instance_meta A JOIN instance_raid B ON A.id = B.instance_meta_id",
            |mut row| InstanceMeta {
                instance_meta_id: row.take(0).unwrap(),
                server_id: row.take(1).unwrap(),
                start_ts: row.take(2).unwrap(),
                end_ts: row.take_opt(3).unwrap().ok(),
                expired: row.take_opt(4).unwrap().ok(),
                map_id: row.take(5).unwrap(),
                participants: Vec::new(),
                instance_specific: MetaType::Raid {
                    map_difficulty: row.take::<u8, usize>(6).unwrap(),
                },
                uploaded_user: row.take(7).unwrap(),
            },
        )
        .into_iter()
        .for_each(|result| {
            instance_metas.insert(result.instance_meta_id, result);
        });

    // Rated Arenas
    // TODO: Rename team_change1 to team1_change
    db_main
        .select(
            "SELECT A.id, A.server_id, A.start_ts, A.end_ts, A.expired, A.map_id, B.winner, B.team_id1, B.team_id2, B.team_change1, B.team_change2, A.uploaded_user FROM instance_meta A JOIN instance_rated_arena B ON A.id = B.instance_meta_id",
            |mut row| {
                (
                    row.take::<u32, usize>(0).unwrap(),
                    row.take::<u32, usize>(1).unwrap(),
                    row.take::<u64, usize>(2).unwrap(),
                    row.take_opt::<u64, usize>(3).unwrap().ok(),
                    row.take_opt::<u64, usize>(4).unwrap().ok(),
                    row.take::<u16, usize>(5).unwrap(),
                    row.take::<u8, usize>(6).unwrap().to_winner(),
                    row.take::<u32, usize>(7).unwrap(),
                    row.take::<u32, usize>(8).unwrap(),
                    row.take::<i32, usize>(9).unwrap(),
                    row.take::<i32, usize>(10).unwrap(),
                    row.take::<u32, usize>(11).unwrap(),
                )
            },
        )
        .into_iter()
        .for_each(|(instance_meta_id, server_id, start_ts, end_ts, expired, map_id, winner, team_id1, team_id2, team1_change, team2_change, uploaded_user)| {
            instance_metas.insert(
                instance_meta_id,
                InstanceMeta {
                    instance_meta_id,
                    server_id,
                    start_ts,
                    end_ts,
                    map_id,
                    expired,
                    participants: Vec::new(),
                    instance_specific: MetaType::RatedArena {
                        winner,
                        team1: armory.get_arena_team_by_id(db_main, team_id1).expect("Foreign key constraint"),
                        team2: armory.get_arena_team_by_id(db_main, team_id2).expect("Foreign key constraint"),
                        team1_change,
                        team2_change,
                    },
                    uploaded_user,
                },
            );
        });

    // Skirmishes
    db_main
        .select(
            "SELECT A.id, A.server_id, A.start_ts, A.end_ts, A.expired, A.map_id, B.winner, A.uploaded_user FROM instance_meta A JOIN instance_skirmish B ON A.id = B.instance_meta_id",
            |mut row| InstanceMeta {
                instance_meta_id: row.take(0).unwrap(),
                server_id: row.take(1).unwrap(),
                start_ts: row.take(2).unwrap(),
                end_ts: row.take_opt(3).unwrap().ok(),
                expired: row.take_opt(4).unwrap().ok(),
                map_id: row.take(5).unwrap(),
                participants: Vec::new(),
                instance_specific: MetaType::Skirmish {
                    winner: row.take::<u8, usize>(6).unwrap().to_winner(),
                },
                uploaded_user: row.take(7).unwrap(),
            },
        )
        .into_iter()
        .for_each(|result| {
            instance_metas.insert(result.instance_meta_id, result);
        });

    // Battlegrounds
    db_main
        .select(
            "SELECT A.id, A.server_id, A.start_ts, A.end_ts, A.expired, A.map_id, B.winner, B.score_alliance, B.score_horde, A.uploaded_user FROM instance_meta A JOIN instance_battleground B ON A.id = B.instance_meta_id",
            |mut row| InstanceMeta {
                instance_meta_id: row.take(0).unwrap(),
                server_id: row.take(1).unwrap(),
                start_ts: row.take(2).unwrap(),
                end_ts: row.take_opt(3).unwrap().ok(),
                expired: row.take_opt(4).unwrap().ok(),
                map_id: row.take(5).unwrap(),
                participants: Vec::new(),
                instance_specific: MetaType::Battleground {
                    winner: row.take::<u8, usize>(6).unwrap().to_winner(),
                    score_alliance: row.take(7).unwrap(),
                    score_horde: row.take(8).unwrap(),
                },
                uploaded_user: row.take(9).unwrap(),
            },
        )
        .into_iter()
        .for_each(|result| {
            instance_metas.insert(result.instance_meta_id, result);
        });

    // Load participants
    db_main
        .select("SELECT A.id, B.character_id FROM instance_meta A JOIN instance_participants B ON A.id = B.instance_meta_id", |mut row| {
            (row.take::<u32, usize>(0).unwrap(), row.take::<u32, usize>(1).unwrap())
        })
        .into_iter()
        .for_each(|(instance_meta_id, character_id)| {
            instance_metas.get_mut(&instance_meta_id).unwrap().participants.push(character_id);
        });
}

trait Winner {
    fn to_winner(&self) -> Option<bool>;
}

impl Winner for u8 {
    fn to_winner(&self) -> Option<bool> {
        // TODO: Find out what these values mean!
        if *self == 0 {
            return None;
        } else if *self == 1 {
            return Some(true);
        }
        Some(false)
    }
}
