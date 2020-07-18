use crate::modules::armory::tools::GetArenaTeam;
use crate::modules::armory::Armory;
use crate::modules::instance::domain_value::{InstanceMeta, MetaType};
use crate::util::database::Select;
use std::collections::HashMap;

pub struct Instance {
    pub instance_metas: HashMap<u32, InstanceMeta>,
}

impl Default for Instance {
    fn default() -> Self {
        Instance { instance_metas: HashMap::new() }
    }
}

impl Instance {
    pub fn init(mut self, db_main: &mut impl Select, armory: &Armory) -> Self {
        self.update_instance_metas(db_main, armory);
        self
    }

    pub fn update_instance_metas(&mut self, db_main: &mut impl Select, armory: &Armory) {
        // Raids
        db_main
            .select("SELECT A.*, B.map_difficulty FROM instance_meta A JOIN instance_raid B ON A.id = B.instance_meta_id", |mut row| InstanceMeta {
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
            })
            .into_iter()
            .for_each(|result| {
                self.instance_metas.insert(result.instance_meta_id, result);
            });

        // Rated Arenas
        // TODO: Rename team_change1 to team1_change
        db_main
            .select(
                "SELECT A.*, B.winner, B.team_id1, B.team_id2, B.team_change1, B.team_change2 FROM instance_meta A JOIN instance_rated_arena B ON A.id = B.instance_meta_id",
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
                    )
                },
            )
            .into_iter()
            .for_each(|(instance_meta_id, server_id, start_ts, end_ts, expired, map_id, winner, team_id1, team_id2, team1_change, team2_change)| {
                self.instance_metas.insert(
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
                    },
                );
            });

        // Skirmishes
        db_main
            .select("SELECT A.*, B.winner FROM instance_meta A JOIN instance_skirmish B ON A.id = B.instance_meta_id", |mut row| InstanceMeta {
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
            })
            .into_iter()
            .for_each(|result| {
                self.instance_metas.insert(result.instance_meta_id, result);
            });

        // Battlegrounds
        db_main
            .select("SELECT A.*, B.winner, B.score_alliance, B.score_horde FROM instance_meta A JOIN instance_battleground B ON A.id = B.instance_meta_id", |mut row| {
                InstanceMeta {
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
                }
            })
            .into_iter()
            .for_each(|result| {
                self.instance_metas.insert(result.instance_meta_id, result);
            });

        // Load participants
        db_main
            .select("SELECT A.id, B.character_id FROM instance_meta A JOIN instance_participants B ON A.id = B.instance_meta_id", |mut row| {
                (row.take::<u32, usize>(0).unwrap(), row.take::<u32, usize>(1).unwrap())
            })
            .into_iter()
            .for_each(|(instance_meta_id, character_id)| {
                self.instance_metas.get_mut(&instance_meta_id).unwrap().participants.push(character_id);
            });
    }
}

trait Winner {
    fn to_winner(self) -> Option<bool>;
}

impl Winner for u8 {
    fn to_winner(self) -> Option<bool> {
        // TODO: Find out what these values mean!
        if self == 0 {
            return None;
        } else if self == 1 {
            return Some(true);
        }
        Some(false)
    }
}
