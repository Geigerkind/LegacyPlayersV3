use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Loot};

pub trait MapLoot {
  fn to_loot(&self) -> Result<Loot, LiveDataProcessorFailure>;
}

impl MapLoot for [u8] {
  fn to_loot(&self) -> Result<Loot, LiveDataProcessorFailure> {
    unimplemented!()
  }
}