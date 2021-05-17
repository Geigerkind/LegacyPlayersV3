use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::material::Cachable;
use crate::modules::armory::Armory;
use crate::modules::armory::tools::GetArenaTeam;
use crate::modules::instance::domain_value::{InstanceAttempt, InstanceMeta, MetaType};
use crate::modules::instance::dto::{InstanceViewerAttempt, RankingResult, SpeedRun};
use crate::params;
use crate::util::database::Select;
use crate::modules::instance::tools::FindInstanceGuild;

pub struct Instance {
    pub instance_metas: Arc<RwLock<HashMap<u32, InstanceMeta>>>,
    pub instance_exports: Arc<RwLock<HashMap<(u32, u8), Cachable<Vec<(u32, String)>>>>>,
    pub instance_attempts: Arc<RwLock<HashMap<u32, Cachable<Vec<InstanceViewerAttempt>>>>>,
    pub speed_runs: Arc<RwLock<Vec<SpeedRun>>>,

    // encounter_id => character_id => Vec<Ranking>
    pub instance_rankings_dps: Arc<RwLock<(u32, HashMap<u32, HashMap<u32, Vec<RankingResult>>>)>>,
    pub instance_rankings_hps: Arc<RwLock<(u32, HashMap<u32, HashMap<u32, Vec<RankingResult>>>)>>,
    pub instance_rankings_tps: Arc<RwLock<(u32, HashMap<u32, HashMap<u32, Vec<RankingResult>>>)>>,
    // attempt_id => (instance_meta_id => Vec<Attempt>)
    pub instance_kill_attempts: Arc<RwLock<(u32, HashMap<u32, Vec<InstanceAttempt>>)>>,
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
            instance_kill_attempts: Arc::new(RwLock::new((0, HashMap::new()))),
            speed_runs: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Instance {
    pub fn init(self, mut db_main: (impl Select + Send + 'static)) -> Self {
        let instance_metas_arc_clone = Arc::clone(&self.instance_metas);
        let instance_exports_arc_clone = Arc::clone(&self.instance_exports);
        let instance_attempts_arc_clone = Arc::clone(&self.instance_attempts);
        let instance_rankings_dps_arc_clone = Arc::clone(&self.instance_rankings_dps);
        let instance_rankings_hps_arc_clone = Arc::clone(&self.instance_rankings_hps);
        let instance_rankings_tps_arc_clone = Arc::clone(&self.instance_rankings_tps);
        let instance_kill_attempts_clone = Arc::clone(&self.instance_kill_attempts);
        let speed_runs_clone = Arc::clone(&self.speed_runs);
        std::thread::spawn(move || {
            let mut armory_counter = 1;
            let mut armory = Armory::default().init(&mut db_main);
            let mut instance_encounters = HashMap::new();
            instance_encounters.insert(409, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
            instance_encounters.insert(249, vec![11]);
            instance_encounters.insert(309, vec![12, 13, 14, 15, 16, 17, 18, 19, 20, 21]);
            instance_encounters.insert(469, vec![22, 23, 24, 25, 26, 27, 28, 29]);
            instance_encounters.insert(509, vec![30, 31, 32, 33, 34, 35]);
            instance_encounters.insert(531, vec![36, 37, 38, 39, 40, 41, 42, 163, 164, 165]);
            instance_encounters.insert(533, vec![43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57]);
            instance_encounters.insert(532, vec![58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68]);
            instance_encounters.insert(565, vec![69, 70]);
            instance_encounters.insert(544, vec![71]);
            instance_encounters.insert(550, vec![72, 73, 74, 75]);
            instance_encounters.insert(548, vec![76, 77, 78, 79, 80, 81]);
            instance_encounters.insert(568, vec![82, 83, 84, 85, 86, 87]);
            instance_encounters.insert(534, vec![88, 89, 90, 91, 92]);
            instance_encounters.insert(564, vec![93, 94, 95, 96, 97, 98, 99, 100, 101]);
            instance_encounters.insert(580, vec![102, 103, 104, 105, 106, 107]);
            instance_encounters.insert(615, vec![108]);
            instance_encounters.insert(616, vec![109]);
            instance_encounters.insert(624, vec![110, 111, 112, 113]);
            instance_encounters.insert(603, vec![114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126]);
            instance_encounters.insert(649, vec![128, 129, 130, 131, 132]);
            instance_encounters.insert(631, vec![133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144]);
            instance_encounters.insert(724, vec![145]);

            loop {
                evict_attempts_cache(Arc::clone(&instance_attempts_arc_clone));
                evict_export_cache(Arc::clone(&instance_exports_arc_clone));
                update_instance_kill_attempts(Arc::clone(&instance_kill_attempts_clone), &mut db_main);
                update_instance_metas(Arc::clone(&instance_metas_arc_clone), &mut db_main, &armory);
                update_instance_rankings_dps(Arc::clone(&instance_rankings_dps_arc_clone), &mut db_main);
                update_instance_rankings_hps(Arc::clone(&instance_rankings_hps_arc_clone), &mut db_main);
                update_instance_rankings_tps(Arc::clone(&instance_rankings_tps_arc_clone), &mut db_main);
                calculate_speed_runs(Arc::clone(&instance_metas_arc_clone),
                                     Arc::clone(&instance_kill_attempts_clone),
                                     Arc::clone(&speed_runs_clone),
                                     &instance_encounters, &mut db_main, &armory);

                if armory_counter % 30 == 0 {
                    armory = Armory::default().init(&mut db_main);
                }
                armory_counter += 1;
                std::thread::sleep(std::time::Duration::from_secs(15));
            }
        });
        self
    }

    pub fn update_instance_meta(&self, db_main: &mut impl Select, armory: &Armory) {
        update_instance_metas(Arc::clone(&self.instance_metas), db_main, armory);
    }
}

fn calculate_speed_runs(instance_metas: Arc<RwLock<HashMap<u32, InstanceMeta>>>,
                        instance_kill_attempts: Arc<RwLock<(u32, HashMap<u32, Vec<InstanceAttempt>>)>>,
                        speed_runs: Arc<RwLock<Vec<SpeedRun>>>,
                        instance_encounters: &HashMap<u16, Vec<u32>>,
                        db_main: &mut impl Select, armory: &Armory) {
    let kill_attempts = instance_kill_attempts.read().unwrap();
    let instance_metas = instance_metas.read().unwrap();
    let mut speed_runs = speed_runs.write().unwrap();
    let already_calculated_speed_runs: Vec<u32> = speed_runs.iter().map(|speed_run| speed_run.instance_meta_id).collect();

    for (instance_meta_id, attempts) in kill_attempts.1.iter().filter(|(im_id, _)| !already_calculated_speed_runs.contains(*im_id)) {
        if !instance_metas.contains_key(instance_meta_id) {
            continue;
        }
        let instance_meta = instance_metas.get(instance_meta_id).unwrap();
        let has_killed_all_encounters = instance_encounters.get(&instance_meta.map_id).unwrap()
            .iter().all(|encounter_id| attempts.iter().any(|attempt| attempt.encounter_id == *encounter_id));

        if !has_killed_all_encounters {
            continue;
        }

        let start = attempts.iter().map(|attempt| attempt.start_ts).min().unwrap();
        let end = attempts.iter().map(|attempt| attempt.end_ts).max().unwrap();
        let (guild_id, guild_name) = instance_meta.participants
            .find_instance_guild(db_main, armory, instance_meta.end_ts.unwrap_or(instance_meta.start_ts))
            .map(|guild| (guild.id, guild.name)).unwrap_or((0, "Pug Raid".to_string()));

        speed_runs.push(SpeedRun {
            instance_meta_id: *instance_meta_id,
            map_id: instance_meta.map_id,
            guild_id,
            guild_name,
            server_id: instance_meta.server_id,
            duration: end - start,
        });
    }
}

fn update_instance_kill_attempts(instance_kill_attempts: Arc<RwLock<(u32, HashMap<u32, Vec<InstanceAttempt>>)>>, db_main: &mut impl Select) {
    let mut kill_attempts = instance_kill_attempts.write().unwrap();
    db_main.select_wparams("SELECT A.instance_meta_id, A.id, A.encounter_id, A.start_ts, A.end_ts FROM instance_attempt A WHERE A.is_kill = 1 AND A.id >= :saved_attempt_id",
                           |mut row|
                               (row.take::<u32, usize>(0).unwrap(), InstanceAttempt {
                                   attempt_id: row.take(1).unwrap(),
                                   encounter_id: row.take(2).unwrap(),
                                   start_ts: row.take(3).unwrap(),
                                   end_ts: row.take(4).unwrap(),
                                   is_kill: true,
                               }), params!("saved_attempt_id" => kill_attempts.0))
        .into_iter()
        .for_each(|(instance_meta_id, instance_attempt)| {
            kill_attempts.0 = instance_attempt.attempt_id;
            let attempt_container = kill_attempts.1.entry(instance_meta_id).or_insert_with(Vec::new);
            attempt_container.push(instance_attempt);
        });
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
        .filter(|(_, cachable)| cachable.get_last_access() + 300 < now)
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
            "SELECT A.id, A.server_id, A.start_ts, A.end_ts, A.expired, A.map_id, B.map_difficulty, C.member_id, A.upload_id FROM instance_meta A JOIN instance_raid B ON A.id = B.instance_meta_id JOIN instance_uploads C ON A.upload_id = C.id",
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
                upload_id: row.take(8).unwrap(),
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
            "SELECT A.id, A.server_id, A.start_ts, A.end_ts, A.expired, A.map_id, B.winner, B.team_id1, B.team_id2, B.team_change1, B.team_change2, C.member_id, A.upload_id FROM instance_meta A JOIN instance_rated_arena B ON A.id = B.instance_meta_id JOIN instance_uploads C ON A.upload_id = C.id",
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
                    row.take::<u32, usize>(12).unwrap(),
                )
            },
        )
        .into_iter()
        .for_each(|(instance_meta_id, server_id, start_ts, end_ts, expired, map_id, winner, team_id1, team_id2, team1_change, team2_change, uploaded_user, upload_id)| {
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
                    upload_id,
                },
            );
        });

    // Skirmishes
    db_main
        .select(
            "SELECT A.id, A.server_id, A.start_ts, A.end_ts, A.expired, A.map_id, B.winner, C.member_id, A.upload_id FROM instance_meta A JOIN instance_skirmish B ON A.id = B.instance_meta_id JOIN instance_uploads C ON A.upload_id = C.id",
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
                upload_id: row.take(8).unwrap(),
            },
        )
        .into_iter()
        .for_each(|result| {
            instance_metas.insert(result.instance_meta_id, result);
        });

    // Battlegrounds
    db_main
        .select(
            "SELECT A.id, A.server_id, A.start_ts, A.end_ts, A.expired, A.map_id, B.winner, B.score_alliance, B.score_horde, C.member_id, A.upload_id FROM instance_meta A JOIN instance_battleground B ON A.id = B.instance_meta_id JOIN instance_uploads C ON A.upload_id = C.id",
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
                upload_id: row.take(10).unwrap(),
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
