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
    pub fn init(mut self, db_main: &mut impl Select) -> Self {
        self.update_instance_metas(db_main);
        self
    }

    pub fn update_instance_metas(&mut self, db_main: &mut impl Select) {
        // Raids
        db_main
            .select("SELECT A.*, B.map_difficulty FROM instance_meta A JOIN instance_raid B ON A.instance_meta_id = B.instance_meta_id", |mut row| InstanceMeta {
                instance_meta_id: row.take(0).unwrap(),
                server_id: row.take(1).unwrap(),
                start_ts: row.take(2).unwrap(),
                end_ts: row.take_opt(3).unwrap().ok(),
                expired: row.take(4).unwrap(),
                map_id: row.take(5).unwrap(),
                instance_specific: MetaType::Raid {
                    map_difficulty: row.take::<u8, usize>(6).unwrap(),
                },
            })
            .into_iter()
            .for_each(|result| {
                self.instance_metas.insert(result.instance_meta_id, result);
            });

        // Rated Arenas
        db_main
            .select(
                "SELECT A.*, B.winner, B.team_id1, B.team_id2, B.team_change1, B.team_change2 FROM instance_meta A JOIN instance_rated_arena B ON A.instance_meta_id = B.instance_meta_id",
                |mut row| InstanceMeta {
                    instance_meta_id: row.take(0).unwrap(),
                    server_id: row.take(1).unwrap(),
                    start_ts: row.take(2).unwrap(),
                    end_ts: row.take_opt(3).unwrap().ok(),
                    expired: row.take(4).unwrap(),
                    map_id: row.take(5).unwrap(),
                    instance_specific: MetaType::RatedArena {
                        winner: row.take::<u8, usize>(6).unwrap().to_winner(),
                        team_id1: row.take(7).unwrap(),
                        team_id2: row.take(8).unwrap(),
                        team_change1: row.take(9).unwrap(),
                        team_change2: row.take(10).unwrap(),
                    },
                },
            )
            .into_iter()
            .for_each(|result| {
                self.instance_metas.insert(result.instance_meta_id, result);
            });

        // Skirmishes
        db_main
            .select("SELECT A.*, B.winner FROM instance_meta A JOIN instance_skirmish B ON A.instance_meta_id = B.instance_meta_id", |mut row| InstanceMeta {
                instance_meta_id: row.take(0).unwrap(),
                server_id: row.take(1).unwrap(),
                start_ts: row.take(2).unwrap(),
                end_ts: row.take_opt(3).unwrap().ok(),
                expired: row.take(4).unwrap(),
                map_id: row.take(5).unwrap(),
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
            .select(
                "SELECT A.*, B.winner, B.score_alliance, B.score_horde FROM instance_meta A JOIN instance_battleground B ON A.instance_meta_id = B.instance_meta_id",
                |mut row| InstanceMeta {
                    instance_meta_id: row.take(0).unwrap(),
                    server_id: row.take(1).unwrap(),
                    start_ts: row.take(2).unwrap(),
                    end_ts: row.take_opt(3).unwrap().ok(),
                    expired: row.take(4).unwrap(),
                    map_id: row.take(5).unwrap(),
                    instance_specific: MetaType::Battleground {
                        winner: row.take::<u8, usize>(6).unwrap().to_winner(),
                        score_alliance: row.take(7).unwrap(),
                        score_horde: row.take(8).unwrap(),
                    },
                },
            )
            .into_iter()
            .for_each(|result| {
                self.instance_metas.insert(result.instance_meta_id, result);
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
