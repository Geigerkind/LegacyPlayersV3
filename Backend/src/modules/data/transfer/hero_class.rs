use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::domain_value::HeroClass;
use crate::modules::data::tools::RetrieveHeroClass;

#[openapi]
#[get("/hero_class/<id>")]
pub fn get_hero_class(me: State<Data>, id: u8) -> Option<Json<HeroClass>>
{
  me.get_hero_class(id)
    .and_then(|hero_class| Some(Json(hero_class)))
}

#[openapi]
#[get("/hero_class")]
pub fn get_all_hero_classes(me: State<Data>) -> Json<Vec<HeroClass>>
{
  Json(me.get_all_hero_classes())
}