use crate::dto::{ApplyFilter, ApplyFilterTs, SearchResult};
use crate::modules::armory::Armory;
use crate::modules::instance::domain_value::{InstanceMeta, MetaType};
use crate::modules::instance::dto::{MetaRaidSearch, RaidSearchFilter};
use crate::modules::instance::tools::{ExportMeta, FindInstanceGuild};
use crate::modules::instance::Instance;
use crate::rpll_table_sort;
use crate::util::ordering::NegateOrdExt;
use std::cmp::Ordering;

pub trait MetaSearch {
    fn search_meta_raids(&self, armory: &Armory, filter: RaidSearchFilter) -> SearchResult<MetaRaidSearch>;
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
                        return Some(MetaRaidSearch {
                            map_id: raid.map_id,
                            map_difficulty,
                            guild: raid.participants.find_instance_guild(armory).map(|guild| guild.name), // TODO: Use Guild DTO!
                            server_id: raid.server_id,
                            start_ts: raid.start_ts,
                            end_ts: raid.end_ts,
                        });
                    }
                }
                None
            })
            // TODO: Guild (filter_map)
            .collect::<Vec<MetaRaidSearch>>();
        let num_characters = result.len();

        result.sort_by(|l_instance, r_instance| {
            rpll_table_sort! {
                (filter.map_id, Some(l_instance.map_id), Some(r_instance.map_id)),
                (filter.map_difficulty, Some(l_instance.map_difficulty), Some(r_instance.map_difficulty)),
                (filter.guild, l_instance.guild.as_ref(), r_instance.guild.as_ref()),
                (filter.server_id, Some(l_instance.server_id), Some(r_instance.server_id)),
                (filter.start_ts, Some(l_instance.start_ts), Some(r_instance.start_ts)),
                (filter.end_ts, l_instance.end_ts, r_instance.end_ts)
            }
        });

        SearchResult {
            result: result.into_iter().skip((filter.page * 10) as usize).take(10).collect(),
            num_items: num_characters,
        }
    }
}
