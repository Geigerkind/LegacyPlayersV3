use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, InstanceArena};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapInstanceArena {
  fn to_instance_arena(&self) -> Result<InstanceArena, LiveDataProcessorFailure>;
}

impl MapInstanceArena for [u8] {
  fn to_instance_arena(&self) -> Result<InstanceArena, LiveDataProcessorFailure> {
    if self.len() != 24 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(InstanceArena {
      map_id: byte_reader::read_u32(&self[0..4])?,
      instance_id: byte_reader::read_u32(&self[4..8])?,
      winner: self[8],
      team_id1: byte_reader::read_u32(&self[9..13])?,
      team_id2: byte_reader::read_u32(&self[13..17])?,
      team_change1: byte_reader::read_i32(&self[17..21])?,
      team_change2: byte_reader::read_i32(&self[21..24])?
    })
  }
}