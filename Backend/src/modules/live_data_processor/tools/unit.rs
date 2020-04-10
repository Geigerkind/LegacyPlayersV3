use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::domain_value;
use crate::modules::live_data_processor::dto;

pub trait MapUnit {
  fn to_unit(&self) -> Result<domain_value::Unit, LiveDataProcessorFailure>;
}

impl MapUnit for dto::Unit {
  fn to_unit(&self) -> Result<domain_value::Unit, LiveDataProcessorFailure> {
    // TODO: Temporary implementation
    Ok(domain_value::Unit::Creature(domain_value::Creature {
      creature_id: self.unit_id as u32,
      entry: 42,
      owner: None
    }))
  }
}