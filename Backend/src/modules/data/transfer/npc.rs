use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::domain_value::NPC;
use crate::modules::data::tools::RetrieveNPC;
use crate::modules::data::Data;

#[openapi]
#[get("/npc/<expansion_id>/<npc_id>")]
pub fn get_npc(me: State<Data>, expansion_id: u8, npc_id: u32) -> Option<Json<NPC>> {
    me.get_npc(expansion_id, npc_id)
        .map(Json)
}
