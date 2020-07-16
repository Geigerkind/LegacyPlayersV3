use crate::dto::SearchResult;
use crate::modules::instance::dto::{MetaRaidSearch, RaidSearchFilter};
use crate::modules::instance::tools::MetaSearch;
use crate::modules::instance::Instance;
use rocket::State;
use rocket_contrib::json::Json;

#[openapi]
#[post("/meta_search/raids", format = "application/json", data = "<filter>")]
pub fn export_raids(me: State<Instance>, filter: Json<RaidSearchFilter>) -> Json<SearchResult<MetaRaidSearch>> {
    Json(me.search_meta_raids(filter.into_inner()))
}
