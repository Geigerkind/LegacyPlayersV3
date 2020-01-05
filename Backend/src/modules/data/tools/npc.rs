use crate::modules::data::Data;
use crate::modules::data::domain_value::NPC;

pub trait RetrieveNPC {
  fn get_npc(&self, expansion_id: u8, npc_id: u32) -> Option<NPC>;
}

impl RetrieveNPC for Data {
  fn get_npc(&self, expansion_id: u8, npc_id: u32) -> Option<NPC> {
    if expansion_id == 0 {
      return None;
    }

    self.npcs.get(expansion_id as usize - 1)
      .and_then(|map| map.get(&npc_id)
        .and_then(|npc| Some(npc.clone())))
  }
}