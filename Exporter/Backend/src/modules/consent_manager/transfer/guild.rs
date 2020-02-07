use rocket::State;

use crate::dto::Failure;
use crate::modules::consent_manager::tools::GuildConsent;
use crate::modules::ConsentManager;

// TODO: Account authentication
#[post("/guild/<guild_id>/<character_id>")]
pub fn give_consent(me: State<ConsentManager>, guild_id: u32, character_id: u32) -> Result<(), Failure>
{
  me.give_consent(guild_id, character_id)
}

// TODO: Account authentication
#[delete("/guild/<guild_id>/<character_id>")]
pub fn withdraw_consent(me: State<ConsentManager>, guild_id: u32, character_id: u32) -> Result<(), Failure>
{
  me.withdraw_consent(guild_id, character_id)
}