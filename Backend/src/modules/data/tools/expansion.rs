use crate::modules::data::Data;
use crate::modules::data::domain_value::Expansion;

pub trait RetrieveExpansion {
  fn get_expansion(&self, id: u8) -> Option<Expansion>;
}

impl RetrieveExpansion for Data {
  fn get_expansion(&self, id: u8) -> Option<Expansion> {
    self.expansions.get(&id)
      .and_then(|expansion| Some(expansion.clone()))
  }
}
