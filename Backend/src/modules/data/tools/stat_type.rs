use crate::modules::data::Data;
use crate::modules::data::domain_value::StatType;

pub trait RetrieveStatType {
  fn get_stat_type(&self, id: u8) -> Option<StatType>;
  fn get_all_stat_types(&self) -> Vec<StatType>;
}

impl RetrieveStatType for Data {
  fn get_stat_type(&self, id: u8) -> Option<StatType> {
    self.stat_types.get(&id)
      .and_then(|stat_type| Some(stat_type.clone()))
  }

  fn get_all_stat_types(&self) -> Vec<StatType> {
    self.stat_types.iter().map(|(_, stat_type)| stat_type.clone()).collect()
  }
}
