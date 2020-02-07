use rocket::State;

use crate::dto::Failure;
use crate::modules::consent_manager::tools::CharacterConsent;
use crate::modules::ConsentManager;
use crate::modules::consent_manager::guard::Authenticate;

// TODO: Account authentication
#[post("/character/<character_id>")]
pub fn give_consent(me: State<ConsentManager>, _auth: Authenticate, character_id: u32) -> Result<(), Failure>
{
  me.give_consent(character_id)
}

// TODO: Account authentication
#[delete("/character/<character_id>")]
pub fn withdraw_consent(me: State<ConsentManager>, _auth: Authenticate, character_id: u32) -> Result<(), Failure>
{
  me.withdraw_consent(character_id)
}