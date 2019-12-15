use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::account::dto::Failure;
use crate::modules::account::material::{Account, APIToken};
use crate::modules::account::tools::Forgot;

#[get("/forgot/<id>")]
pub fn receive_confirmation(me: State<Account>, id: String) -> Result<Json<APIToken>, Failure>
{
  me.recv_forgot_password(&id)
    .and_then(|api_token| Ok(Json(api_token)))
}

#[post("/forgot", data = "<mail>", format = "application/json")]
pub fn send_confirmation(me: State<Account>, mail: Json<String>) -> Result<(), Failure>
{
  me.send_forgot_password(&mail)
}