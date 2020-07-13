#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum MetaType {
    Raid { map_difficulty: u8 },
    RatedArena { winner: Option<bool>, team_id1: u32, team_id2: u32, team_change1: i32, team_change2: i32 },
    Skirmish { winner: Option<bool> },
    Battleground { winner: Option<bool>, score_alliance: u32, score_horde: u32 },
}

impl MetaType {
    pub fn to_u8(&self) -> u8 {
        match self {
            MetaType::Raid { .. } => 0,
            MetaType::RatedArena { .. } => 1,
            MetaType::Skirmish { .. } => 2,
            MetaType::Battleground { .. } => 3,
        }
    }
}
