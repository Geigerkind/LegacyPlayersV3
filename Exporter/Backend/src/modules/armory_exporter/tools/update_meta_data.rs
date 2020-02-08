use crate::modules::ArmoryExporter;
use mysql_connection::tools::Execute;

pub trait UpdateMetaData {
  fn update_meta_data(&self);
}

impl UpdateMetaData for ArmoryExporter {
  fn update_meta_data(&self) {
    self.db_lp_consent.execute_wparams("UPDATE meta_data SET last_fetch=:last_fetch", params!(
      "last_fetch" => self.last_fetch_time
    ));
  }
}