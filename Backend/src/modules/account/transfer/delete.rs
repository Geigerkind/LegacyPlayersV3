use rocket::State;

use crate::modules::account::{dto::Failure, guard::Authenticate, material::Account, tools::Delete};

#[openapi]
#[get("/delete/<id>")]
pub fn confirm(me: State<Account>, id: String) -> Result<(), Failure> {
    me.confirm_delete(&id)
}

#[openapi]
#[delete("/delete")]
pub fn request(me: State<Account>, auth: Authenticate) -> Result<(), Failure> {
    me.issue_delete(auth.0)
}
