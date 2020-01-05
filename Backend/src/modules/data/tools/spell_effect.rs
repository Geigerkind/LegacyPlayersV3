use crate::modules::data::Data;
use crate::modules::data::domain_value::SpellEffect;

pub trait RetrieveSpellEffect {
  fn get_spell_effects(&self, expansion_id: u8, spell_id: u32) -> Option<Vec<SpellEffect>>;
}

impl RetrieveSpellEffect for Data {
  fn get_spell_effects(&self, expansion_id: u8, spell_id: u32) -> Option<Vec<SpellEffect>> {
    if expansion_id == 0 {
      return None;
    }

    self.spell_effects.get(expansion_id as usize - 1)
        .and_then(|map| map.get(&spell_id).and_then(|spell_effects| Some(spell_effects.clone())))
  }
}