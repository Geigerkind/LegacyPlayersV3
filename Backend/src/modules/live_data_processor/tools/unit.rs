use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::domain_value::Unit;

pub trait MapUnit {
  fn to_unit(&self) -> Result<Unit, LiveDataProcessorFailure>;
}

impl MapUnit for u64 {
  fn to_unit(&self) -> Result<Unit, LiveDataProcessorFailure> {
    unimplemented!()
  }
}