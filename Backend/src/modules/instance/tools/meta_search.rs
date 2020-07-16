use crate::dto::{ApplyFilter, ApplyFilterTs, SearchResult};
use crate::modules::armory::dto::SearchGuildDto;
use crate::modules::armory::Armory;
use crate::modules::instance::domain_value::{InstanceMeta, MetaType};
use crate::modules::instance::dto::{MetaRaidSearch, MetaRatedArenaSearch, RaidSearchFilter, RatedArenaSearchFilter, SearchArenaTeam};
use crate::modules::instance::tools::{ExportMeta, FindInstanceGuild};
use crate::modules::instance::Instance;
use crate::rpll_table_sort;
use crate::util::ordering::NegateOrdExt;
use std::cmp::Ordering;

pub trait MetaSearch {
    fn search_meta_raids(&self, armory: &Armory, filter: RaidSearchFilter) -> SearchResult<MetaRaidSearch>;
    fn search_meta_rated_arenas(&self, filter: RatedArenaSearchFilter) -> SearchResult<MetaRatedArenaSearch>;
}

impl MetaSearch for Instance {
    fn search_meta_raids(&self, armory: &Armory, mut filter: RaidSearchFilter) -> SearchResult<MetaRaidSearch> {
        filter.guild.convert_to_lowercase();
        let mut result = self
            .export_meta(0)
            .into_iter()
            .filter(|raid| filter.map_id.apply_filter(raid.map_id))
            .filter(|raid| filter.server_id.apply_filter(raid.server_id))
            .filter(|raid| filter.start_ts.apply_filter_ts(raid.start_ts))
            .filter(|raid| filter.end_ts.apply_filter_ts(raid.end_ts))
            .filter_map(|raid| {
                if let InstanceMeta {
                    instance_specific: MetaType::Raid { map_difficulty },
                    ..
                } = raid
                {
                    if filter.map_difficulty.apply_filter(map_difficulty) {
                        let guild = raid.participants.find_instance_guild(armory).map(|guild| SearchGuildDto { guild_id: guild.id, name: guild.name });
                        if filter.guild.apply_filter(guild.as_ref().map(|guild| guild.name.clone())) {
                            return Some(MetaRaidSearch {
                                map_id: raid.map_id,
                                map_difficulty,
                                guild,
                                server_id: raid.server_id,
                                start_ts: raid.start_ts,
                                end_ts: raid.end_ts,
                            });
                        }
                    }
                }
                None
            })
            .collect::<Vec<MetaRaidSearch>>();
        let num_raids = result.len();

        result.sort_by(|l_instance, r_instance| {
            rpll_table_sort! {
                (filter.map_id, Some(l_instance.map_id), Some(r_instance.map_id)),
                (filter.map_difficulty, Some(l_instance.map_difficulty), Some(r_instance.map_difficulty)),
                (filter.guild, l_instance.guild.as_ref().map(|guild| guild.name.clone()), r_instance.guild.as_ref().map(|guild| guild.name.clone())),
                (filter.server_id, Some(l_instance.server_id), Some(r_instance.server_id)),
                (filter.start_ts, Some(l_instance.start_ts), Some(r_instance.start_ts)),
                (filter.end_ts, l_instance.end_ts, r_instance.end_ts)
            }
        });

        SearchResult {
            result: result.into_iter().skip((filter.page * 10) as usize).take(10).collect(),
            num_items: num_raids,
        }
    }

    fn search_meta_rated_arenas(&self, mut filter: RatedArenaSearchFilter) -> SearchResult<MetaRatedArenaSearch> {
        filter.team1.convert_to_lowercase();
        filter.team2.convert_to_lowercase();
        let mut result = self
            .export_meta(0)
            .into_iter()
            .filter(|rated_arena| filter.map_id.apply_filter(rated_arena.map_id))
            .filter(|rated_arena| filter.server_id.apply_filter(rated_arena.server_id))
            .filter(|rated_arena| filter.start_ts.apply_filter_ts(rated_arena.start_ts))
            .filter(|rated_arena| filter.end_ts.apply_filter_ts(rated_arena.end_ts))
            .filter_map(|rated_arena| {
                if let InstanceMeta {
                    instance_specific: MetaType::RatedArena { team1, team2, team1_change, team2_change, .. },
                    ..
                } = rated_arena
                {
                    if filter.team1_change.apply_filter(team1_change) && filter.team2_change.apply_filter(team2_change) && filter.team1.apply_filter(Some(team1.team_name.clone())) && filter.team2.apply_filter(Some(team2.team_name.clone())) {
                        return Some(MetaRatedArenaSearch {
                            map_id: rated_arena.map_id,
                            team1: SearchArenaTeam { team_id: team1.id, name: team1.team_name },
                            team2: SearchArenaTeam { team_id: team2.id, name: team2.team_name },
                            server_id: rated_arena.server_id,
                            team1_change,
                            team2_change,
                            start_ts: rated_arena.start_ts,
                            end_ts: rated_arena.end_ts,
                        });
                    }
                }
                None
            })
            .collect::<Vec<MetaRatedArenaSearch>>();
        let num_rated_arenas = result.len();

        result.sort_by(|l_instance, r_instance| {
            rpll_table_sort! {
                (filter.map_id, Some(l_instance.map_id), Some(r_instance.map_id)),
                (filter.team1, Some(l_instance.team1.name.clone()), Some(r_instance.team1.name.clone())),
                (filter.team2, Some(l_instance.team2.name.clone()), Some(r_instance.team2.name.clone())),
                (filter.server_id, Some(l_instance.server_id), Some(r_instance.server_id)),
                (filter.team1_change, Some(l_instance.team1_change), Some(r_instance.team1_change)),
                (filter.team2_change, Some(l_instance.team2_change), Some(r_instance.team2_change)),
                (filter.start_ts, Some(l_instance.start_ts), Some(r_instance.start_ts)),
                (filter.end_ts, l_instance.end_ts, r_instance.end_ts)
            }
        });

        SearchResult {
            result: result.into_iter().skip((filter.page * 10) as usize).take(10).collect(),
            num_items: num_rated_arenas,
        }
    }
}
