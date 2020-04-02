use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, AuraApplication};

pub trait MapAuraApplication {
  fn to_aura_application(&self) -> Result<AuraApplication, LiveDataProcessorFailure>;
}

impl MapAuraApplication for [u8] {
  fn to_aura_application(&self) -> Result<AuraApplication, LiveDataProcessorFailure> {
    unimplemented!()
  }
}