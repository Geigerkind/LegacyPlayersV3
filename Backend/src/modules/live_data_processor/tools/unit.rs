use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::domain_value::{Unit, Creature};

pub trait MapUnit {
  fn to_unit(&self) -> Result<Unit, LiveDataProcessorFailure>;
}

impl MapUnit for u64 {
  fn to_unit(&self) -> Result<Unit, LiveDataProcessorFailure> {
    // TODO: Temporary implementation
    Ok(Unit::Creature(Creature {
      creature_id: *self as u32,
      entry: 42,
      owner: None
    }))
  }
}