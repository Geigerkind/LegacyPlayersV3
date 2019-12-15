use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::account::dto::{CreateMember, Failure};
use crate::modules::account::guard::Authenticate;
use crate::modules::account::material::{Account, APIToken};
use crate::modules::account::tools::Create;

#[post("/create", format = "application/json", data = "<params>")]
pub fn create(me: State<Account>, params: Json<CreateMember>) -> Result<Json<APIToken>, Failure>
{
  me.create(&params.credentials.mail, &params.nickname, &params.credentials.password)
    .and_then(|api_token| Ok(Json(api_token)))
}

#[get("/create/<id>")]
pub fn confirm(me: State<Account>, id: String) -> Result<(), Failure>
{
  if me.confirm(&id) {
    return Ok(());
  }
  Err(Failure::Unknown)
}

#[post("/create/resend", format = "application/json")]
pub fn resend_confirm(me: State<Account>, auth: Authenticate) -> Result<(), Failure>
{
  if me.send_confirmation(auth.0) {
    return Ok(());
  }
  Err(Failure::Unknown)
}
