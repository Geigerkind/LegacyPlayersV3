use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::instance::dto::SpeedKill;
use crate::modules::instance::Instance;

#[openapi]
#[get("/speed_kill")]
pub fn get_speed_kills(me: State<Instance>) -> Json<Vec<SpeedKill>> {
    let speed_kills = me.speed_kills.read().unwrap();
    Json(speed_kills.clone())
}