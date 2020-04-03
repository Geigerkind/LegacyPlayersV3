use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Loot};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapLoot {
  fn to_loot(&self) -> Result<Loot, LiveDataProcessorFailure>;
}

impl MapLoot for [u8] {
  fn to_loot(&self) -> Result<Loot, LiveDataProcessorFailure> {
    Ok(Loot {
      unit: byte_reader::read_u64(&self[0..8])?,
      item_id: byte_reader::read_u32(&self[8..12])?
    })
  }
}