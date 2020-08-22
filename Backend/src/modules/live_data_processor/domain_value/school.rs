#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[repr(u8)]
pub enum School {
    Physical = 0x00,
    Holy = 0x01,
    Fire = 0x02,
    Nature = 0x04,
    Frost = 0x08,
    Shadow = 0x10,
    Arcane = 0x20,
}
