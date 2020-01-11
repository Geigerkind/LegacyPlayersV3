use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::tools::RetrieveServer;
use crate::modules::data::dto::AvailableServer;

#[openapi]
#[get("/server/<id>")]
pub fn get_server(me: State<Data>, id: u32) -> Option<Json<AvailableServer>>
{
  me.get_server(id)
    .and_then(|server| Some(Json(server)))
}

#[openapi]
#[get("/server")]
pub fn get_all_servers(me: State<Data>) -> Json<Vec<AvailableServer>>
{
  Json(me.get_all_servers())
}