use crate::modules::instance::dto::SpeedRun;
use crate::modules::instance::Instance;
use rocket::State;
use rocket_contrib::json::Json;

#[openapi]
#[get("/speed_run")]
pub fn get_speed_runs(me: State<Instance>) -> Json<Vec<SpeedRun>> {
    let speed_runs = me.speed_runs.read().unwrap();
    Json(speed_runs.clone())
}