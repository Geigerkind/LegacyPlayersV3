use crate::modules::live_data_processor::Event;

use crate::modules::instance::dto::InstanceFailure;
use crate::modules::instance::tools::ExportInstance;
use crate::modules::instance::Instance;
use rocket::State;
use rocket_contrib::json::Json;

#[openapi]
#[get("/<server_id>/<instance_meta_id>/<event_type>")]
pub fn get_instance_event_type(me: State<Instance>, server_id: u32, instance_meta_id: u32, event_type: u8) -> Result<Json<Vec<Event>>, InstanceFailure> {
    me.export_instance_event_type(server_id, instance_meta_id, event_type).map(Json)
}
