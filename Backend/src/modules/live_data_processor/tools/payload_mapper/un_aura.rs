use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, UnAura};

pub trait MapUnAura {
  fn to_un_aura(&self) -> Result<UnAura, LiveDataProcessorFailure>;
}

impl MapUnAura for [u8] {
  fn to_un_aura(&self) -> Result<UnAura, LiveDataProcessorFailure> {
    unimplemented!()
  }
}