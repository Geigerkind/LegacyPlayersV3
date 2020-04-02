pub struct InstanceBattleground {
  pub map_id: u32,
  pub instance_id: u32,
  pub winner: Option<u8>,
  pub score_alliance: Option<u32>,
  pub score_horde: Option<u32>
}