#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum HitType {
    Evade = 0,
    Miss = 1,
    Dodge = 2,
    Block = 3,
    Parry = 4,
    Glancing = 5,
    Crit = 6,
    Crushing = 7,
    Hit = 8,
    Resist = 9,
    Immune = 10,
    Environment = 11,
    Absorb = 12,
    Interrupted = 13,
    Undefined = 255,
}

impl HitType {
    pub fn from_u8(number: u8) -> Self {
        match number {
            0 => HitType::Evade,
            1 => HitType::Miss,
            2 => HitType::Dodge,
            3 => HitType::Block,
            4 => HitType::Parry,
            5 => HitType::Glancing,
            6 => HitType::Crit,
            7 => HitType::Crushing,
            8 => HitType::Hit,
            9 => HitType::Resist,
            10 => HitType::Immune,
            11 => HitType::Environment,
            12 => HitType::Absorb,
            13 => HitType::Interrupted,

            // Should not happen!
            _ => HitType::Undefined,
        }
    }
}
