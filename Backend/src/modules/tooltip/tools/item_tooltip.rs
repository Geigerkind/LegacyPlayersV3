use crate::modules::tooltip::domain_value::ItemTooltip;
use crate::dto::Failure;
use crate::modules::tooltip::Tooltip;

pub trait RetrieveItemTooltip {
  fn get_item(&self, item_id: u32) -> Result<ItemTooltip, Failure>;
  fn get_character_item(&self, item_id: u32, character_history_id: u32) -> Result<ItemTooltip, Failure>;
}

impl RetrieveItemTooltip for Tooltip {
  fn get_item(&self, item_id: u32) -> Result<ItemTooltip, Failure> {
    unimplemented!()
  }

  fn get_character_item(&self, item_id: u32, character_history_id: u32) -> Result<ItemTooltip, Failure> {
    unimplemented!()
  }
}