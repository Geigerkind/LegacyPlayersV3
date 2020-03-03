use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::armory::Armory;
use crate::modules::tooltip::dto::TooltipFailure;
use crate::modules::tooltip::Tooltip;
use crate::modules::tooltip::material::GuildTooltip;
use crate::modules::tooltip::tools::RetrieveGuildTooltip;

#[openapi]
#[get("/guild/<id>")]
pub fn get_guild(me: State<Tooltip>, armory: State<Armory>, id: u32) -> Result<Json<GuildTooltip>, TooltipFailure>
{
    me.get_guild(&armory, id).and_then(|tooltip| Ok(Json(tooltip)))
}
