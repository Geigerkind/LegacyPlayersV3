use crate::modules::live_data_processor::dto::{AuraApplication, LiveDataProcessorFailure};
use crate::modules::live_data_processor::tools::byte_reader;
use crate::modules::live_data_processor::tools::payload_mapper::unit::MapUnit;

pub trait MapAuraApplication {
    fn to_aura_application(&self) -> Result<AuraApplication, LiveDataProcessorFailure>;
}

impl MapAuraApplication for [u8] {
    fn to_aura_application(&self) -> Result<AuraApplication, LiveDataProcessorFailure> {
        if self.len() != 25 {
            return Err(LiveDataProcessorFailure::InvalidInput);
        }
        Ok(AuraApplication {
            caster: self[0..8].to_unit()?,
            target: self[8..16].to_unit()?,
            spell_id: byte_reader::read_u32(&self[16..20]).unwrap(),
            stack_amount: byte_reader::read_u32(&self[20..24]).unwrap(),
            applied: self[24] == 1,
        })
    }
}
