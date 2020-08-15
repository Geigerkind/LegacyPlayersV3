use crate::modules::instance::domain_value::InstanceMeta;
use crate::modules::instance::tools::ExportMeta;
use crate::modules::instance::Instance;
use rocket::State;
use rocket_contrib::json::Json;

#[openapi]
#[get("/meta/raids")]
pub fn export_raids(me: State<Instance>) -> Json<Vec<InstanceMeta>> {
    Json(me.export_meta(0))
}

#[openapi]
#[get("/meta/rated_arenas")]
pub fn export_rated_arenas(me: State<Instance>) -> Json<Vec<InstanceMeta>> {
    Json(me.export_meta(1))
}

#[openapi]
#[get("/meta/skirmishes")]
pub fn export_skirmishes(me: State<Instance>) -> Json<Vec<InstanceMeta>> {
    Json(me.export_meta(2))
}

#[openapi]
#[get("/meta/battlegrounds")]
pub fn export_battlegrounds(me: State<Instance>) -> Json<Vec<InstanceMeta>> {
    Json(me.export_meta(3))
}
