use crate::modules::armory::material::Guild;
use crate::modules::armory::tools::{GetCharacter, GetGuild};
use crate::modules::armory::Armory;
use grouping_by::GroupingBy;

pub trait FindInstanceGuild {
    fn find_instance_guild(&self, armory: &Armory) -> Option<Guild>;
}

// Where u32 is a character_id
impl FindInstanceGuild for Vec<u32> {
    fn find_instance_guild(&self, armory: &Armory) -> Option<Guild> {
        let same_guild_fraction = 0.75;
        for (guild_id, group) in &self
            .iter()
            .map(|character_id| {
                if let Some(character) = armory.get_character(*character_id) {
                    if let Some(character_history) = character.last_update {
                        return character_history.character_guild.map(|inner| inner.guild_id);
                    }
                }
                None
            })
            .grouping_by(|character_guild_id| *character_guild_id)
        {
            if (group.len() as f64) / (self.len() as f64) >= same_guild_fraction {
                return guild_id.and_then(|guild_id| armory.get_guild(guild_id));
            }
        }
        None
    }
}
