use crate::modules::ArmoryExporter;
use mysql_connection::tools::Select;
use crate::modules::transport_layer::InstanceReset;

pub trait RetrieveMetaInstanceReset {
  fn get_instance_reset(&self) -> Vec<InstanceReset>;
}

impl RetrieveMetaInstanceReset for ArmoryExporter {
  fn get_instance_reset(&self) -> Vec<InstanceReset> {
    self.db_characters.select(
      "SELECT mapid, difficulty, resettime FROM instance_reset", &|mut row| {
        InstanceReset {
          map_id: row.take(0).unwrap(),
          difficulty: row.take(1).unwrap(),
          reset_time: row.take(2).unwrap(),
        }
      })
  }
}