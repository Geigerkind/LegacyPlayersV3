use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Interrupt};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapInterrupt {
  fn to_interrupt(&self) -> Result<Interrupt, LiveDataProcessorFailure>;
}

impl MapInterrupt for [u8] {
  fn to_interrupt(&self) -> Result<Interrupt, LiveDataProcessorFailure> {
    if self.len() != 12 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(Interrupt {
      target: byte_reader::read_u64(&self[0..8])?,
      interrupted_spell_id: byte_reader::read_u32(&self[8..12])?
    })
  }
}