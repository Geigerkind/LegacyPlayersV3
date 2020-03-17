use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::{dto::AvailableServer, tools::RetrieveServer, Data};

#[openapi]
#[get("/server/<id>")]
pub fn get_server(me: State<Data>, id: u32) -> Option<Json<AvailableServer>> {
    me.get_server(id).map(Json)
}

#[openapi]
#[get("/server")]
pub fn get_all_servers(me: State<Data>) -> Json<Vec<AvailableServer>> {
    Json(me.get_all_servers())
}
