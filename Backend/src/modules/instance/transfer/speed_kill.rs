use crate::modules::instance::Instance;
use rocket::State;
use rocket_contrib::json::Json;
use crate::modules::instance::domain_value::InstanceAttempt;


#[openapi]
#[get("/speed_kill")]
pub fn get_speed_kills(me: State<Instance>) -> Json<Vec<InstanceAttempt>> {
    let speed_kills = me.instance_kill_attempts.read().unwrap();
    Json(speed_kills.1.iter().fold(Vec::new(), |mut acc, (_, attempts)| {
        acc.append(&mut attempts.clone());
        acc
    }))
}