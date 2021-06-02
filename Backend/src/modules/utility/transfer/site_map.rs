use rocket::response::content::Xml;
use rocket::State;

use crate::modules::armory::Armory;
use crate::modules::data::Data;
use crate::modules::utility::Utility;
use crate::modules::utility::tools::SiteMap;

#[openapi]
#[get("/site_map_gen/<page>")]
pub fn build_character_site_map(me: State<Utility>, data: State<Data>, armory: State<Armory>, page: u32) -> Xml<String> {
    Xml(me.build_character_site_map(&armory, &data, page))
}