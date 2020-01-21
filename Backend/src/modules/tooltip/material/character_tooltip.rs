use crate::modules::tooltip::domain_value::CharacterGuild;

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct CharacterTooltip {
    pub name: String,
    pub server: String,
    pub guild: Option<CharacterGuild>,
}