use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::account::{
    domain_value::AccountInformation,
    dto::Failure,
    guard::Authenticate,
    material::{APIToken, Account},
    tools::Update,
};

#[openapi]
#[post("/update/password", format = "application/json", data = "<content>")]
pub fn password(me: State<Account>, auth: Authenticate, content: Json<String>) -> Result<Json<APIToken>, Failure> {
    me.change_password(&content, auth.0).map(Json)
}

#[openapi]
#[post("/update/nickname", format = "application/json", data = "<content>")]
pub fn nickname(me: State<Account>, auth: Authenticate, content: Json<String>) -> Result<Json<AccountInformation>, Failure> {
    me.change_name(&content, auth.0).map(Json)
}

#[openapi]
#[post("/update/mail", format = "application/json", data = "<content>")]
pub fn request_mail(me: State<Account>, auth: Authenticate, content: Json<String>) -> Result<Json<bool>, Failure> {
    me.request_change_mail(&content, auth.0).map(Json)
}

#[openapi]
#[get("/update/mail/<id>")]
pub fn confirm_mail(me: State<Account>, id: String) -> Result<Json<APIToken>, Failure> {
    me.confirm_change_mail(&id).map(Json)
}
