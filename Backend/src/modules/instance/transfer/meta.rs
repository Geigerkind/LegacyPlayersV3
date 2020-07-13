use crate::modules::instance::domain_value::InstanceMeta;
use crate::modules::instance::dto::InstanceFailure;
use crate::modules::instance::tools::ExportMeta;
use crate::modules::instance::Instance;
use rocket::State;
use rocket_contrib::json::Json;

#[openapi]
#[get("/meta/raids")]
pub fn export_raids(me: State<Instance>) -> Result<Json<Vec<InstanceMeta>>, InstanceFailure> {
    me.export_meta(0).map(Json)
}

#[openapi]
#[get("/meta/rated_arenas")]
pub fn export_rated_arenas(me: State<Instance>) -> Result<Json<Vec<InstanceMeta>>, InstanceFailure> {
    me.export_meta(1).map(Json)
}

#[openapi]
#[get("/meta/skirmishes")]
pub fn export_skirmishes(me: State<Instance>) -> Result<Json<Vec<InstanceMeta>>, InstanceFailure> {
    me.export_meta(2).map(Json)
}

#[openapi]
#[get("/meta/battlegrounds")]
pub fn export_battlegrounds(me: State<Instance>) -> Result<Json<Vec<InstanceMeta>>, InstanceFailure> {
    me.export_meta(3).map(Json)
}
