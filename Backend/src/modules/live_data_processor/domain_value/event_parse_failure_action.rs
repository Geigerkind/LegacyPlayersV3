#[derive(Debug)]
pub enum EventParseFailureAction {
    DiscardFirst,
    DiscardAll,
    Wait
}