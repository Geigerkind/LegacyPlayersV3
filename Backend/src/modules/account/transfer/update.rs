use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::account::{
    domain_value::AccountInformation,
    dto::Failure,
    guard::Authenticate,
    material::{APIToken, Account},
    tools::Update,
};
use crate::MainDb;

#[openapi(skip)]
#[post("/update/password", format = "application/json", data = "<content>")]
pub fn password(mut db_main: MainDb, me: State<Account>, auth: Authenticate, content: Json<String>) -> Result<Json<APIToken>, Failure> {
    me.change_password(&mut *db_main, &content, auth.0).map(Json)
}

#[openapi(skip)]
#[post("/update/nickname", format = "application/json", data = "<content>")]
pub fn nickname(mut db_main: MainDb, me: State<Account>, auth: Authenticate, content: Json<String>) -> Result<Json<AccountInformation>, Failure> {
    me.change_name(&mut *db_main, &content, auth.0).map(Json)
}

#[openapi]
#[post("/update/mail", format = "application/json", data = "<content>")]
pub fn request_mail(me: State<Account>, auth: Authenticate, content: Json<String>) -> Result<Json<bool>, Failure> {
    me.request_change_mail(&content, auth.0).map(Json)
}

#[openapi(skip)]
#[get("/update/mail/<id>")]
pub fn confirm_mail(mut db_main: MainDb, me: State<Account>, id: String) -> Result<Json<APIToken>, Failure> {
    me.confirm_change_mail(&mut *db_main, &id).map(Json)
}
