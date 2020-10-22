use crate::modules::data::tools::RetrieveSpell;
use crate::modules::data::Data;

pub fn parse_spell_args(data: &Data, spell_name: &str) -> Option<u32> {
    data.get_spell_by_name(1, &spell_name.to_string()).map(|spell| spell.id)
}
