use crate::modules::utility::domain_value::Paste;
use crate::modules::utility::Utility;

pub trait RetrieveAddonPaste {
    fn get_addon_paste(&self, id: u32) -> Option<Paste>;
    fn get_addon_pastes(&self) -> Vec<Paste>;
}

impl RetrieveAddonPaste for Utility {
    fn get_addon_paste(&self, id: u32) -> Option<Paste> {
        let addon_pastes = self.addon_pastes.read().unwrap();
        addon_pastes.get(&id).cloned()
    }

    fn get_addon_pastes(&self) -> Vec<Paste> {
        let addon_pastes = self.addon_pastes.read().unwrap();
        addon_pastes.iter().map(|(_, paste)| paste.clone()).collect()
    }
}