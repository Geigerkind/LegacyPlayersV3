use rocket::State;

use crate::modules::ConsentManager;
use crate::dto::Failure;

#[post("/<character_id>", format = "application/json")]
pub fn give_consent(me: State<ConsentManager>, character_id: u32) -> Result<(), Failure>
{
  unimplemented!()
}

#[delete("/<character_id>", format = "application/json")]
pub fn withdraw_consent(me: State<ConsentManager>, character_id: u32) -> Result<(), Failure>
{
  unimplemented!()
}