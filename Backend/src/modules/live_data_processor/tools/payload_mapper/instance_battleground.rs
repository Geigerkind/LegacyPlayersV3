use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, InstanceBattleground};

pub trait MapInstanceBattleground {
  fn to_instance_battleground(&self) -> Result<InstanceBattleground, LiveDataProcessorFailure>;
}

impl MapInstanceBattleground for [u8] {
  fn to_instance_battleground(&self) -> Result<InstanceBattleground, LiveDataProcessorFailure> {
    unimplemented!()
  }
}