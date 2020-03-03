use crate::modules::armory::Armory;
use crate::modules::armory::dto::{ArmoryFailure, GuildViewerDto};
use crate::modules::data::Data;

pub trait GuildViewer {
    fn get_guild_view(&self, data: &Data, language_id: u8, guild_id: u32) -> Result<GuildViewerDto, ArmoryFailure>;
}

impl GuildViewer for Armory {
    fn get_guild_view(&self, data: &Data, language_id: u8, guild_id: u32) -> Result<GuildViewerDto, ArmoryFailure> {
        unimplemented!()
    }
}