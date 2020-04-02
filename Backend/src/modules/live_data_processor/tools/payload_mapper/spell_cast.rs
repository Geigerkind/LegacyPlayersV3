use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, SpellCast};

pub trait MapSpellCast {
  fn to_spell_cast(&self) -> Result<SpellCast, LiveDataProcessorFailure>;
}

impl MapSpellCast for [u8] {
  fn to_spell_cast(&self) -> Result<SpellCast, LiveDataProcessorFailure> {
    unimplemented!()
  }
}