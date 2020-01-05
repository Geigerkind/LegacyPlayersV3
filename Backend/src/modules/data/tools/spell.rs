use crate::modules::data::Data;
use crate::modules::data::domain_value::Spell;

pub trait RetrieveSpell {
  fn get_spell(&self, expansion_id: u8, spell_id: u32) -> Option<Spell>;
}

impl RetrieveSpell for Data {
  fn get_spell(&self, expansion_id: u8, spell_id: u32) -> Option<Spell> {
    if expansion_id == 0 {
      return None;
    }

    self.spells.get(expansion_id as usize - 1).and_then(|map| map.get(&spell_id).and_then(|spell| Some(spell.clone())))
  }
}