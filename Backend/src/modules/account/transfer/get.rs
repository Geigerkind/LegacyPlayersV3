use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::account::domain_value::AccountInformation;
use crate::modules::account::dto::Failure;
use crate::modules::account::guard::Authenticate;
use crate::modules::account::material::Account;
use crate::modules::account::tools::GetAccountInformation;

#[openapi]
#[get("/get")]
pub fn get_account_information(me: State<Account>, auth: Authenticate) -> Result<Json<AccountInformation>, Failure>
{
  me.get(auth.0)
    .and_then(|acc_info| Ok(Json(acc_info)))
}