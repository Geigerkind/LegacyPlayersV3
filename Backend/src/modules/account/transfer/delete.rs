use rocket::State;

use crate::modules::account::dto::Failure;
use crate::modules::account::guard::Authenticate;
use crate::modules::account::material::Account;
use crate::modules::account::tools::Delete;

#[openapi]
#[get("/delete/<id>")]
pub fn confirm(me: State<Account>, id: String) -> Result<(), Failure>
{
  me.confirm_delete(&id)
}

#[openapi]
#[delete("/delete")]
pub fn request(me: State<Account>, auth: Authenticate) -> Result<(), Failure>
{
  me.issue_delete(auth.0)
}

