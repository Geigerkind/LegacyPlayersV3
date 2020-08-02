#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum School {
    Physical = 0,
    Holy = 1,
    Fire = 2,
    Nature = 3,
    Frost = 4,
    Shadow = 5,
    Arcane = 6,
    Undefined = 255,
}

impl Default for School {
    fn default() -> Self {
        School::Undefined
    }
}

impl School {
    pub fn from_u8(number: u8) -> Self {
        match number {
            0 => School::Physical,
            1 => School::Holy,
            2 => School::Fire,
            3 => School::Nature,
            4 => School::Frost,
            5 => School::Shadow,
            6 => School::Arcane,

            // Should never happen!
            _ => School::Undefined,
        }
    }
}
