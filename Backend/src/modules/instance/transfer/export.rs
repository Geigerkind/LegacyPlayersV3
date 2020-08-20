use crate::modules::live_data_processor::Event;

use crate::modules::armory::Armory;
use crate::modules::instance::dto::{InstanceFailure, InstanceViewerAttempt, InstanceViewerMeta, InstanceViewerParticipant};
use crate::modules::instance::tools::ExportInstance;
use crate::modules::instance::Instance;
use crate::MainDb;
use rocket::State;
use rocket_contrib::json::Json;

#[openapi]
#[get("/export/<instance_meta_id>/<event_type>/<last_event_id>")]
pub fn get_instance_event_type(me: State<Instance>, instance_meta_id: u32, event_type: u8, last_event_id: u32) -> Result<Json<Vec<Event>>, InstanceFailure> {
    me.export_instance_event_type(instance_meta_id, event_type)
        .map(|events| events.into_iter().filter(|event| event.id > last_event_id).take(5000).collect())
        .map(Json)
}

#[openapi]
#[get("/export/<instance_meta_id>")]
pub fn get_instance_meta(me: State<Instance>, armory: State<Armory>, instance_meta_id: u32) -> Result<Json<InstanceViewerMeta>, InstanceFailure> {
    me.get_instance_meta(&armory, instance_meta_id).map(Json)
}

#[openapi]
#[get("/export/participants/<instance_meta_id>")]
pub fn get_instance_participants(me: State<Instance>, armory: State<Armory>, instance_meta_id: u32) -> Result<Json<Vec<InstanceViewerParticipant>>, InstanceFailure> {
    me.get_instance_participants(&armory, instance_meta_id).map(Json)
}

#[openapi]
#[get("/export/attempts/<instance_meta_id>")]
pub fn get_instance_attempts(me: State<Instance>, mut db_main: MainDb, instance_meta_id: u32) -> Result<Json<Vec<InstanceViewerAttempt>>, InstanceFailure> {
    me.get_instance_attempts(&mut (*db_main), instance_meta_id).map(Json)
}
