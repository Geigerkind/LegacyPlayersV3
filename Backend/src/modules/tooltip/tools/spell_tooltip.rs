use crate::modules::tooltip::domain_value::SpellTooltip;
use crate::dto::Failure;
use crate::modules::tooltip::Tooltip;
use crate::modules::data::Data;

pub trait RetrieveSpellTooltip {
  fn get_spell(&self, data: &Data, language_id: u8, expansion_id: u8, spell_id: u32) -> Result<SpellTooltip, Failure>;
}

impl RetrieveSpellTooltip for Tooltip {
  fn get_spell(&self, data: &Data, language_id: u8, expansion_id: u8, spell_id: u32) -> Result<SpellTooltip, Failure> {
    unimplemented!()
  }
}