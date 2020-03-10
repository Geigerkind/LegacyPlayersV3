use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::domain_value::Gem;
use crate::modules::data::tools::RetrieveGem;
use crate::modules::data::Data;

#[openapi]
#[get("/gem/<expansion_id>/<gem_id>")]
pub fn get_gem(me: State<Data>, expansion_id: u8, gem_id: u32) -> Option<Json<Gem>> {
    me.get_gem(expansion_id, gem_id)
        .and_then(|result| Some(Json(result)))
}
