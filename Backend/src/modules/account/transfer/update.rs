use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::account::domain_value::AccountInformation;
use crate::modules::account::dto::Failure;
use crate::modules::account::guard::Authenticate;
use crate::modules::account::material::{Account, APIToken};
use crate::modules::account::tools::Update;

#[post("/update/password", format = "application/json", data = "<content>")]
pub fn password(me: State<Account>, auth: Authenticate, content: Json<String>) -> Result<Json<APIToken>, Failure> {
  me.change_password(&content, auth.0)
    .and_then(|api_token| Ok(Json(api_token)))
}

#[post("/update/nickname", format = "application/json", data = "<content>")]
pub fn nickname(me: State<Account>, auth: Authenticate, content: Json<String>) -> Result<Json<AccountInformation>, Failure> {
  me.change_name(&content, auth.0)
    .and_then(|acc_info| Ok(Json(acc_info)))
}

#[post("/update/mail", format = "application/json", data = "<content>")]
pub fn request_mail(me: State<Account>, auth: Authenticate, content: Json<String>) -> Result<Json<bool>, Failure> {
  me.request_change_mail(&content, auth.0)
    .and_then(|changed_password| Ok(Json(changed_password)))
}

#[get("/update/mail/<id>")]
pub fn confirm_mail(me: State<Account>, id: String) -> Result<Json<APIToken>, Failure> {
  me.confirm_change_mail(&id)
    .and_then(|api_token| Ok(Json(api_token)))
}