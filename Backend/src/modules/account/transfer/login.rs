use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::account::dto::Credentials;
use crate::modules::account::dto::Failure;
use crate::modules::account::material::{APIToken, Account};
use crate::modules::account::tools::Login;

#[openapi]
#[post("/login", format = "application/json", data = "<params>")]
pub fn login(me: State<Account>, params: Json<Credentials>) -> Result<Json<APIToken>, Failure> {
    me.login(&params.mail, &params.password)
        .and_then(|api_token| Ok(Json(api_token)))
}
