pub struct VanillaWoWCBTLParser {
    pub you: String,
}

impl VanillaWoWCBTLParser {
    pub fn new(you: String) -> Self {
        VanillaWoWCBTLParser {
            you,
        }
    }
}