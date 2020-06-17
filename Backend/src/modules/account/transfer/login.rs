use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::account::{
    dto::{Credentials, Failure},
    material::{APIToken, Account},
    tools::Login,
};

#[openapi]
#[post("/login", format = "application/json", data = "<params>")]
pub fn login(me: State<Account>, params: Json<Credentials>) -> Result<Json<APIToken>, Failure> {
    me.login(&params.mail, &params.password).map(Json)
}
