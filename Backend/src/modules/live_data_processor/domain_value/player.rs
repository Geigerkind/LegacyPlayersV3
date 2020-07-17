use crate::modules::armory::material::Character;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Player {
    pub character_id: u32,
    pub server_uid: u64,
    #[serde(skip)]
    pub character: Option<Character>, // Is none if it is deserialized
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.character_id == other.character_id
    }
}
