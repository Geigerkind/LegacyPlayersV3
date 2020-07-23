use crate::dto::SearchResult;
use crate::modules::armory::Armory;
use crate::modules::instance::dto::{BattlegroundSearchFilter, MetaBattlegroundSearch, MetaRaidSearch, MetaRatedArenaSearch, MetaSkirmishSearch, RaidSearchFilter, RatedArenaSearchFilter, SkirmishSearchFilter};
use crate::modules::instance::tools::MetaSearch;
use crate::modules::instance::Instance;
use rocket::State;
use rocket_contrib::json::Json;
use crate::modules::data::Data;

#[openapi]
#[post("/meta_search/raids", format = "application/json", data = "<filter>")]
pub fn export_raids(me: State<Instance>, armory: State<Armory>, data: State<Data>, filter: Json<RaidSearchFilter>) -> Json<SearchResult<MetaRaidSearch>> {
    Json(me.search_meta_raids(&armory, &data, filter.into_inner()))
}

#[openapi]
#[post("/meta_search/rated_arena", format = "application/json", data = "<filter>")]
pub fn export_rated_arenas(me: State<Instance>, filter: Json<RatedArenaSearchFilter>) -> Json<SearchResult<MetaRatedArenaSearch>> {
    Json(me.search_meta_rated_arenas(filter.into_inner()))
}

#[openapi]
#[post("/meta_search/skirmishes", format = "application/json", data = "<filter>")]
pub fn export_skirmishes(me: State<Instance>, filter: Json<SkirmishSearchFilter>) -> Json<SearchResult<MetaSkirmishSearch>> {
    Json(me.search_meta_skirmishes(filter.into_inner()))
}

#[openapi]
#[post("/meta_search/battlegrounds", format = "application/json", data = "<filter>")]
pub fn export_battlegrounds(me: State<Instance>, filter: Json<BattlegroundSearchFilter>) -> Json<SearchResult<MetaBattlegroundSearch>> {
    Json(me.search_meta_battlegrounds(filter.into_inner()))
}
