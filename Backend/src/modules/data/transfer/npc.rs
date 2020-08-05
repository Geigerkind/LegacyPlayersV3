use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::domain_value::Localized;
use crate::modules::data::guard::Language;
use crate::modules::data::tools::RetrieveLocalization;
use crate::modules::data::{domain_value::NPC, tools::RetrieveNPC, Data};

#[openapi]
#[get("/npc/<expansion_id>/<npc_id>")]
pub fn get_npc(me: State<Data>, expansion_id: u8, npc_id: u32) -> Option<Json<NPC>> {
    me.get_npc(expansion_id, npc_id).map(Json)
}

#[openapi]
#[get("/npc/localized/<expansion_id>/<npc_id>")]
pub fn get_npc_localized(me: State<Data>, language: Language, expansion_id: u8, npc_id: u32) -> Option<Json<Localized<NPC>>> {
    me.get_npc(expansion_id, npc_id).map(|npc| {
        Json(Localized {
            localization: me.get_localization(language.0, npc.localization_id).unwrap().content,
            base: npc,
        })
    })
}

#[openapi]
#[get("/npc/localized/bosses")]
pub fn get_boss_npcs(me: State<Data>, language: Language) -> Json<Vec<Localized<NPC>>> {
    Json((vec![
        /* Vanilla */
        // Molten Core
        (1, 12118), (1, 11982), (1, 12259), (1, 12057), (1, 12056), (1, 12264), (1, 12098), (1, 11988), (1, 12018), (1, 11502),
        // Black Wing Lair
        (1, 12435), (1, 13020), (1, 12017), (1, 11983), (1, 14601), (1, 11981), (1, 14020), (1, 11583),
        // Ahn'Qiraj 40
        // TODO: Bug Trio and Twins
        (1, 15263), (1, 15516), (1, 15510), (1, 15509), (1, 15517), (1, 15727), (1, 15299),
        // Ahn'Qiraj 20
        (1, 15348), (1, 15341), (1, 15340), (1, 15370), (1, 15369), (1, 15339),
        // Zul'Gurub
        (1, 14517), (1, 14507), (1, 14510), (1, 11382), (1, 15082), (1, 15083), (1, 15085), (1, 15114), (1, 14509), (1, 14515), (1, 11380), (1, 14834),
        // Naxxramas
        // TODO: 4 Horseman
        (1, 15028), (1, 15931), (1, 15932), (1, 15928), (1, 15956), (1, 15953), (1, 15952), (1, 16061), (1, 16060), (1, 15954), (1, 15936), (1, 16011), (1, 15989), (1, 15990),
        /* TBC */
        // Kara
        // TODO: Chessevent
        (2, 15550), (2, 15687), (2, 15688), (2, 15689), (2, 15690), (2, 15691), (2, 16457), (2, 16524), (2, 17225),
        // Mag
        (2, 17257),
        // SSC
        (2, 21212), (2, 21213), (2, 21214), (2, 21215), (2, 21216), (2, 21217),
        // Tempest Keep
        (2, 18805), (2, 19514), (2, 19516), (2, 19622),
        // Gruuls Lair
        (2, 18831), (2, 19044),
        // Zul'Aman
        (2, 23574), (2, 23576), (2, 23577), (2, 23578), (2, 23863), (2, 24239),
        // Black Temple
        // TODO: Council and Face
        (2, 22841), (2, 22887), (2, 22898), (2, 22917), (2, 22947), (2, 22948), (2, 22871),
        // Hyjal
        (2, 17767), (2, 17808), (2, 17842), (2, 17888), (2, 17968),
        // Sunwell
        // TODO: Twins
        (2, 24882), (2, 24844), (2, 25038), (2, 25741), (2, 25315)
        /* WOTLK */
        // TODO
    ]).into_iter().map(|(expansion_id, npc_id)|
        me.get_npc(expansion_id, npc_id)
            .map(|npc| {
                Localized {
                    localization: me.get_localization(language.0, npc.localization_id).unwrap().content,
                    base: npc,
                }
            })).filter(|npc| npc.is_some())
        .map(|npc| npc.unwrap()).collect())
}