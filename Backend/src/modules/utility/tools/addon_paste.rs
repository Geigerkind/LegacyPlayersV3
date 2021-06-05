use std::str::FromStr;

use crate::modules::utility::domain_value::Paste;
use crate::modules::utility::dto::{PasteDto, UtilityFailure};
use crate::modules::utility::Utility;
use crate::params;
use crate::util::database::{Execute, Select};

pub trait RetrieveAddonPaste {
    fn get_addon_paste(&self, id: u32) -> Option<Paste>;
    fn get_addon_pastes(&self) -> Vec<Paste>;
}

pub trait UpdateAddonPaste {
    fn replace_addon_paste(&self, db_main: &mut (impl Select + Execute), paste: PasteDto) -> Result<u32, UtilityFailure>;
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

impl UpdateAddonPaste for Utility {
    fn replace_addon_paste(&self, db_main: &mut (impl Select + Execute), paste: PasteDto) -> Result<u32, UtilityFailure> {
        if let Some(id) = paste.id {
            let mut addon_pastes = self.addon_pastes.write().unwrap();
            let mut i_paste = addon_pastes.get_mut(&id).ok_or(UtilityFailure::InvalidInput)?;
            i_paste.title = paste.title.clone();
            i_paste.addon_name = paste.addon_name.clone();
            i_paste.expansion_id = paste.expansion_id;
            i_paste.description = paste.description.clone();
            i_paste.content = paste.content.clone();
            db_main.execute_wparams("UPDATE utility_addon_paste SET `title`=:title, `expansion_id`=:expansion_id, \
            `addon_name`=:addon_name, `tags`=:tags, `description`=:description, `content`=:content \
            WHERE `id`=:id", params!("id" => id, "title" => paste.title.clone(), "expansion_id" => paste.expansion_id,
            "addon_name" => paste.addon_name.clone(), "tags" => paste.tags.clone(), "description" => paste.description.clone(), "content" => paste.content));
            Ok(id)
        } else {
            let params = params!("title" => paste.title.clone(), "expansion_id" => paste.expansion_id, "addon_name" => paste.addon_name.clone(),
            "tags" => paste.tags.clone(), "description" => paste.description.clone(), "content" => paste.content.clone());
            if db_main.execute_wparams("INSERT INTO utility_addon_paste (`title`, `expansion_id`, `addon_name`, `tags`, `description`, `content`) \
             VALUES (:title, :expansion_id, :addon_name, :tags, :description, :content)", params) {
                let id: u32 = db_main.select_value("SELECT MAX(id) FROM utility_addon_paste", |mut row| row.take(0).unwrap()).unwrap();
                let mut addon_pastes = self.addon_pastes.write().unwrap();
                addon_pastes.insert(id, Paste {
                    id,
                    title: paste.title.clone(),
                    expansion_id: paste.expansion_id,
                    addon_name: paste.addon_name.clone(),
                    tags: paste.tags.split(",").map(|num| u32::from_str(num).unwrap()).collect(),
                    description: paste.description.clone(),
                    content: paste.content.clone(),
                });
                return Ok(id);
            }
            Err(UtilityFailure::InvalidInput)
        }
    }
}